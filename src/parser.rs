use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use chrono::{DateTime, Datelike, Utc};
use maiq_shared::*;
use regex::Regex;
use scraper::Html;
use table_extract::Row;

use crate::{fetch::Fetched, replacer, ParserError};

#[derive(Debug, Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<u8>,
  pub lesson: Lesson,
}

lazy_static! {
  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);
  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", " ", ""];
}

// todo: rewrite table_extract with tl crate
pub fn parse(fetched: &Fetched) -> Result<Snapshot, ParserError> {
  let table = match table_extract::Table::find_first(&fetched.html) {
    Some(x) => x,
    None => return Err(ParserError::NotYet),
  };
  let mut table = table.iter();
  let mut lessons = vec![];
  let mut prev: Option<ParsedLesson> = None;
  let date = parse_date(&table.next().unwrap());
  for row in table.skip(2) {
    let row = parse_row(&row);
    let lesson = parse_lesson(row, &prev)?;
    if lesson.is_some() {
      prev = lesson.clone();
      lessons.push(lesson.unwrap());
    }
  }

  let groups = map_lessons_to_groups(&lessons, date.0.iso_week().week() % 2 != 0, date.1);

  Ok(Snapshot::new(groups, date.0))
}

fn parse_date(row: &Row) -> (DateTime<Utc>, i64) {
  let full_str_binding = as_text(row.iter().next().unwrap());
  let iter = full_str_binding.trim().split(' ').rev();
  let weekday = iter.skip(2).next().unwrap();
  let date = utils::map_day(&utils::now_date(0), weekday);
  (date.0, date.1)
}

fn parse_lesson(row: Vec<Option<String>>, prev: &Option<ParsedLesson>) -> Result<Option<ParsedLesson>, ParserError> {
  macro_rules! prev {
    () => {
      match prev {
        Some(x) => x,
        None => return Ok(None),
      }
    };
  }

  if row.iter().all(|x| x.is_none()) {
    return Ok(None);
  }

  let (group, subgroup) = match (&row[0], &row[1]) {
    (Some(g), None) => (g.clone(), None),
    (Some(g), Some(s)) => (g.clone(), s.parse::<u8>().ok()),
    (None, Some(s)) => (prev!().group.clone(), s.parse::<u8>().ok()),
    (None, None) => match prev {
      Some(x) => (x.group.clone(), x.lesson.subgroup.clone()),
      None => return Ok(None),
    },
  };

  let nums = match &row[2] {
    Some(x) => x.split(',').map(|x| x.parse::<u8>().unwrap()).collect(),
    None => prev!().nums.clone(),
  };

  let name = match &row[3] {
    Some(x) => x.clone(),
    None => prev!().lesson.name.clone(),
  };

  let teacher = row[4].clone();
  let classroom = row[5].clone();

  let lesson = Lesson { num: 0, subgroup, name, teacher, classroom };
  let parsed = ParsedLesson { group, nums, lesson };
  Ok(Some(parsed))
}

fn parse_row(row: &Row) -> Vec<Option<String>> {
  lazy_static! {
    static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();
    static ref NUM_REGEX: Regex = Regex::new(r#"^(\d{1},{0,1})*$"#).unwrap();
  }

  let mut r = vec![None; 6];

  let mut raw = row.iter().map(|x| as_text(x)).filter(|x| x != " ").peekable();

  if raw.peek() == None {
    return r;
  }

  if GROUP_REGEX.is_match(raw.peek().unwrap()) {
    let binding = raw.next().unwrap();
    let mut iter = binding.split(&[' ', ' ', '\n']);
    r[0] = iter.next().and_then(|x| Some(x.trim().to_owned())); // group
    r[1] = iter.next().and_then(|x| Some(x.trim().to_owned())); // subgroup
  }

  match NUM_REGEX.is_match(raw.peek().unwrap()) {
    true => r[2] = Some(raw.next().unwrap().trim().to_owned()), // num
    false => (),
  }

  {
    let iter = raw.next().unwrap();
    let name_n_teacher = iter.split(',');

    match name_n_teacher.clone().count() {
      1 => r[3] = Some(name_n_teacher.map(|x| x.trim()).collect::<Vec<&str>>().join(", ")),
      count if count > 0 => {
        r[3] = Some(
          name_n_teacher
            .clone()
            .take(count - 1)
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .join(", "),
        );
        r[4] = name_n_teacher.clone().last().and_then(|x| Some(x.trim().to_owned()));
      }
      _ => (),
    };
  }

  r[5] = raw.next().and_then(|x| Some(x.trim().to_owned())); // classroom
  r
}

fn map_lessons_to_groups(vec: &Vec<ParsedLesson>, is_even: bool, date_offset: i64) -> Vec<Group> {
  let mut res: Vec<Group> = vec![];
  for parsed in vec {
    for num in &parsed.nums {
      if *num < 1 || parsed.lesson.name.as_str() == "Нет" {
        continue;
      }
      let name = parsed.group.as_str().clone();
      let group = if let Some(x) = res.iter().position(|x| x.name.as_str() == name) {
        &mut res[x]
      } else {
        res.push(Group::new(name.into()));
        res.last_mut().unwrap()
      };

      if parsed.lesson.name.as_str() == "День самостоятельной работы" {
        group.lessons.push(parsed.lesson.clone());
        continue;
      }

      let mut lesson = if parsed.lesson.name.as_str() == "По расписанию" {
        replacer::replace(*num, &parsed.group, is_even, date_offset)
      } else {
        None
      };

      if parsed.lesson.classroom.is_some() {
        if let Some(l) = lesson.as_mut() {
          l.classroom = parsed.lesson.classroom.clone()
        }
      }

      group.lessons.push(lesson.unwrap_or_else(|| {
        let mut lesson = parsed.lesson.clone();
        lesson.num = *num;
        lesson
      }))
    }
  }

  res.iter_mut().for_each(|g| {
    g.lessons.sort_by_key(|k| k.num);
    g.uid = g.uid()
  });
  res
}

fn as_text(html: &str) -> String {
  let frag = Html::parse_fragment(html);
  CORASICK.replace_all(frag.root_element().text().collect::<String>().as_str(), CORASICK_REPLACE_PATTERNS.as_slice())
}
