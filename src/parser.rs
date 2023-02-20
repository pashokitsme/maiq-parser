use scraper::Html;
use table_extract::Row;

use crate::{replacer, ParserError};
use chrono::{DateTime, Utc};
use maiq_shared::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<u8>,
  pub lesson: Lesson,
}

pub fn parse(html: &String, date: DateTime<Utc>) -> Result<Snapshot, ParserError> {
  let table = table_extract::Table::find_first(&html).ok_or(ParserError::NoTable)?;
  let mut lessons = vec![];
  let mut prev = None;
  for row in table.iter() {
    let row = parse_row(&row);
    let lesson = parse_lesson(row, &prev)?;
    if lesson.is_some() {
      prev = lesson.clone();
      lessons.push(lesson.unwrap());
    }
  }

  let groups = map_lessons_to_groups(lessons, date);

  Ok(Snapshot::new(groups, date))
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

  macro_rules! not_empty {
    ($e: expr) => {
      match $e {
        None => None,
        Some(x) => match x.as_str() {
          "" | " " => None,
          _ => Some(x),
        },
      }
    };
  }

  
  if row.iter().all(|x| x.is_none()) {
    return Ok(None);
  }
  
  let mut row = row.into_iter();

  macro_rules! next {
      () => {
          row.next().unwrap()
      };
  }

  let (group, subgroup) = match (next!(), next!()) {
    (Some(g), None) => (g, None),
    (Some(g), Some(s)) => (g, s.parse::<u8>().ok()),
    (None, Some(s_prev)) => (prev!().group.clone(), s_prev.parse::<u8>().ok()),
    (None, None) => match prev {
      Some(prev) => (prev.group.clone(), prev.lesson.subgroup),
      None => return Ok(None),
    },
  };

  let nums = match next!() {
    Some(x) => x.split(',').map(|x| x.parse::<u8>().unwrap_or(0)).collect(),
    None => prev!().nums.clone(),
  };

  let name = match next!() {
    Some(x) => x.clone(),
    None => prev!().lesson.name.clone(),
  };

  let teacher = not_empty!(next!());
  let classroom = not_empty!(next!());

  let lesson = Lesson { num: 0, subgroup, name, teacher, classroom };
  let parsed = ParsedLesson { group, nums, lesson };
  Ok(Some(parsed))
}

fn parse_row(row: &Row) -> Vec<Option<String>> {
  lazy_static! {
    static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();
    static ref NUM_REGEX: Regex = Regex::new(r#"^(\d{1},{0,1})*$"#).unwrap();
  }

  macro_rules! regex_match_opt {
    ($regex: expr, $pattern: expr) => {
      match $pattern {
        Some(pattern) => $regex.is_match(pattern),
        None => false,
      }
    };
  }

  let mut r = vec![None; 6];
  let mut raw = row.iter().map(|x| into_text(x)).filter(|x| x != " ").peekable();

  if raw.peek() == None {
    return r;
  }

  // println!("{}", raw.clone().map(|x| format!("{};", x)).collect::<String>());

  if regex_match_opt!(GROUP_REGEX, raw.peek()) {
    let binding = raw.next().unwrap();
    let mut iter = binding.split(&[' ', ' ', '\n']).peekable();
    r[0] = iter.next().and_then(|x| Some(x.trim().to_owned())); // group
    r[1] = iter.next().and_then(|x| Some(x.trim().to_owned())); // subgroup
  }

  if regex_match_opt!(NUM_REGEX, raw.peek()) {
    r[2] = Some(raw.next().unwrap().trim().to_owned()) // num
  }

  if let Some(name_n_teacher) = raw.next() {
    let name_n_teacher = name_n_teacher.split(',');

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

fn map_lessons_to_groups(vec: Vec<ParsedLesson>, date: DateTime<Utc>) -> Vec<Group> {
  let mut groups: Vec<Group> = vec![];
  for parsed in vec {
    for num in parsed
      .nums
      .iter()
      .filter(|&num| *num > 0 && parsed.lesson.name != "Нет")
      .map(|num| *num)
    {
      let name = parsed.group.as_str().clone();
      let group = match groups.iter().position(|x| x.name.as_str() == name) {
        Some(x) => &mut groups[x],
        None => {
          groups.push(Group::new(name.into()));
          groups.last_mut().unwrap()
        }
      };

      let mut lesson = match &*parsed.lesson.name {
        "День самостоятельной работы" => {
          group.lessons.push(parsed.lesson.clone());
          continue;
        }
        _ => replacer::replace_or_clone(num, &parsed.group, &parsed.lesson, date),
      };

      if parsed.lesson.classroom.is_some() {
        lesson.classroom = parsed.lesson.classroom.clone()
      }

      group.lessons.push(lesson);
    }
  }

  groups.iter_mut().for_each(|g| {
    g.lessons.sort_by_key(|k| k.num);
    g.uid = g.uid()
  });
  groups
}

fn into_text(html: &str) -> String {
  let fragment = Html::parse_fragment(html);
  let mut res = String::new();

  for text in fragment.root_element().text() {
    let mut chars = text.chars().into_iter().peekable();
    while let Some(c) = chars.next() {
      let next = chars.peek();
      if next.is_some() && next.unwrap().is_whitespace() && c.is_whitespace() {
        continue;
      }
      if next.is_none() && c.is_whitespace() {
        continue;
      }

      match c {
        '\n' => (),
        c if c.is_whitespace() => res.push(' '),
        c => res.push(c),
      }
    }
  }

  res
}
