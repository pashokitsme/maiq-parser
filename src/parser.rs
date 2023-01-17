use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use chrono::{DateTime, Datelike, Utc};
use maiq_shared::*;
use regex::Regex;
use scraper::Html;
use table_extract::Row;

use crate::{fetch::Fetched, replacer, ParserError};

#[derive(Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<u8>,
  pub lesson: Lesson,
}

lazy_static! {
  static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();
  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);
  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", " ", ""];
}

// todo: rewrite table_extract with tl crate
// todo: parse row and then use it instead of parsing parts of it
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
    let lesson = parse_lesson(&row, &prev)?;
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

// ? Idk how it works :(
fn parse_lesson(row: &Row, prev: &Option<ParsedLesson>) -> Result<Option<ParsedLesson>, ParserError> {
  let mut row = row.iter().peekable();
  // println!("{}", row.clone().map(|x| as_text(x)).collect::<String>());
  if as_text(row.peek().unwrap()).is_empty() {
    return Ok(None);
  }

  let (group, subgroup) = if is_group(&as_text(row.peek().unwrap())) {
    let group_n_subgroup = as_text(row.next().unwrap());
    let mut group_n_subgroup = group_n_subgroup.split(&[' ', ' ']);
    let group = group_n_subgroup.next().unwrap().trim();
    let subgroup = match group_n_subgroup.next() {
      Some(x) => x.trim().parse::<u8>().ok(),
      None => None,
    };
    (group.to_string(), subgroup.clone())
  } else {
    let cloned = prev.clone().unwrap();
    (cloned.group, cloned.lesson.subgroup)
  };

  let nums_binding = as_text(&row.next().unwrap());

  let mut nums: Vec<u8> = vec![];
  for num in nums_binding.split(',') {
    let num = match num.parse::<u8>().ok() {
      Some(x) => x - 1,
      None => match prev {
        Some(x) => x.lesson.num,
        None => return Ok(None),
      },
    };
    nums.push(num);
  }

  let name_n_teacher = as_text(row.next().unwrap());
  let classroom = match name_n_teacher.as_str() {
    "Нет" => None,
    _ => match row.next() {
      Some(x) => match as_text(&x.as_str()).as_str() {
        "" | " " | " " => None,
        x => Some(x.to_string()),
      },
      None => None,
    },
  };

  // todo: rewrite this trash code
  let name_n_teacher = name_n_teacher.split(',');
  let count = name_n_teacher.clone().count();
  if count > 1 {
    let mut name = name_n_teacher
      .clone()
      .take(count - 1)
      .map(|s| format!("{s},"))
      .collect::<String>()
      .trim()
      .to_string();
    name.pop();
    let teacher = name_n_teacher.rev().next().and_then(|t| Some(t.trim().to_string()));
    Ok(Some(ParsedLesson { group, nums, lesson: Lesson { num: 0, name, subgroup, teacher, classroom } }))
  } else {
    let name = name_n_teacher.collect::<String>().trim().to_string();
    Ok(Some(ParsedLesson { group, nums, lesson: Lesson { num: 0, name, subgroup, teacher: None, classroom } }))
  }
}

fn map_lessons_to_groups(vec: &Vec<ParsedLesson>, is_even: bool, date_offset: i64) -> Vec<Group> {
  let mut res: Vec<Group> = vec![];
  for parsed in vec {
    for num in &parsed.nums {
      if *num < 1 {
        continue;
      }
      let name = parsed.group.as_str().clone();
      let group = if let Some(x) = res.iter().position(|x| x.name.as_str() == name) {
        &mut res[x]
      } else {
        res.push(Group::new(name.into()));
        res.last_mut().unwrap()
      };

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

  res.iter_mut().for_each(|g| g.lessons.sort_by_key(|k| k.num));
  res
}

fn as_text(html: &str) -> String {
  let frag = Html::parse_fragment(html);
  CORASICK.replace_all(frag.root_element().text().collect::<String>().as_str(), CORASICK_REPLACE_PATTERNS.as_slice())
}

fn is_group(pattern: &str) -> bool {
  GROUP_REGEX.is_match(pattern)
}
