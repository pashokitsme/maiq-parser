use scraper::Html;
use table_extract::Row;

use crate::{replacer, ParserError};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use maiq_shared::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<u8>,
  pub lesson: Lesson,
}

pub fn parse(html: &str, possible_date: DateTime<Utc>) -> Result<Snapshot, ParserError> {
  let table = table_extract::Table::find_first(html).ok_or(ParserError::NoTable)?;
  let mut table = table.into_iter();
  let mut lessons = vec![];
  let mut prev = None;

  let date = {
    let mut date = None;
    for _ in 0..2 {
      if let Some(d) = parse_date(table.next().unwrap()) {
        date = Some(d);
        break;
      }
    }

    let date = date.unwrap_or(possible_date);
    if possible_date > date {
      possible_date
    } else {
      date
    }
  };

  for row in table {
    let row = parse_row(row);
    let lesson = parse_lesson(row, &prev)?;
    if let Some(lesson) = lesson {
      prev = Some(lesson.clone());
      lessons.push(lesson);
    }
  }

  let groups = map_lessons_to_groups(lessons, date);

  Ok(Snapshot::new(groups, date))
}

const MONTHS: [&str; 12] =
  ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];

fn parse_date(row: Row) -> Option<DateTime<Utc>> {
  let content = row.into_iter().map(|x| into_text(x)).collect::<String>();
  let mut split = content.split(' ');

  while let Some(word) = split.next() {
    let day = word.trim().parse::<u32>();
    if day.is_err() {
      continue;
    }

    let day = day.unwrap();

    let month = match split.next() {
      None => continue,
      Some(month) => MONTHS.iter().position(|&m| m == month).map(|x| x as u32 + 1),
    };

    let month = match month {
      Some(m) => m,
      None => continue,
    };

    let year = match split.next() {
      None => continue,
      Some(year) => {
        let y = &year.trim()[..4];
        match y.parse::<i32>() {
          Ok(y) => y,
          Err(_) => continue,
        }
      }
    };

    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    return Some(DateTime::from_utc(NaiveDateTime::new(date, NaiveTime::default()), Utc));
  }

  None
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

  let mut row = row.into_iter().peekable();

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
    Some(x) => x,
    None => prev!().lesson.name.clone(),
  };

  let teacher = not_empty!(next!());
  // println!("{} - {}: {:?}", group, name, row.peek());

  let classroom = not_empty!(next!());

  let lesson = Lesson { num: 0, subgroup, name, teacher, classroom };
  let parsed = ParsedLesson { group, nums, lesson };
  Ok(Some(parsed))
}

fn parse_row(row: Row) -> Vec<Option<String>> {
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
  let mut raw = row
    .into_iter()
    .map(|x| into_text(x))
    .filter(|x| !x.is_empty())
    .peekable();

  if raw.peek().is_none() {
    return r;
  }

  // println!("{}", raw.clone().map(|x| format!("{};", x)).collect::<String>());

  if regex_match_opt!(GROUP_REGEX, raw.peek()) {
    let binding = raw.next().unwrap();
    let mut iter = binding.split(&[' ', ' ', '\n']).peekable();
    r[0] = iter.next().map(|x| x.trim().into()); // group
    r[1] = iter.next().map(|x| x.replace("п/г", "").trim().into()); // subgroup
  }

  if regex_match_opt!(NUM_REGEX, raw.peek()) {
    r[2] = Some(raw.next().unwrap()) // num
  }

  if let Some(name_n_teacher) = raw.next() {
    let name_n_teacher = name_n_teacher.split(',');

    match name_n_teacher.clone().count() {
      1 => r[3] = Some(name_n_teacher.collect::<Vec<&str>>().join(", ")),
      count if count > 0 => {
        r[3] = Some(
          name_n_teacher
            .clone()
            .take(count - 1)
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .join(", "),
        );
        r[4] = name_n_teacher.last().map(|x| x.trim().into());
      }
      _ => (),
    };
  }

  r[5] = raw.next(); // classroom
  r
}

fn map_lessons_to_groups(vec: Vec<ParsedLesson>, date: DateTime<Utc>) -> Vec<Group> {
  let mut groups: Vec<Group> = vec![];
  for parsed in vec.into_iter() {
    for num in parsed
      .nums
      .iter()
      .filter(|&num| *num > 0 && parsed.lesson.name != "Нет" && parsed.lesson.name != "нет")
    {
      let name = parsed.group.as_str();
      let group = match groups.iter().position(|x| x.name == name) {
        Some(x) => &mut groups[x],
        None => {
          groups.push(Group::new(name.into()));
          groups.last_mut().unwrap()
        }
      };

      let lesson = match &*parsed.lesson.name {
        "День самостоятельной работы" => {
          group.lessons.push(parsed.lesson.clone());
          break;
        }
        _ => replacer::replace_or_clone(*num, &parsed.group, &parsed.lesson, date),
      };

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
  let doc = Html::parse_document(html);
  let mut fragments = doc.root_element().text().peekable();
  let mut res = String::new();

  while let Some(fragment) = fragments.next() {
    let mut chars = fragment.chars().peekable();
    let mut whitespaces_only = true;
    while let Some(c) = chars.next() {
      let next = chars.peek();
      if whitespaces_only && !c.is_whitespace() {
        whitespaces_only = false;
      }

      if c.is_whitespace() && ((next.is_some() && next.unwrap().is_whitespace()) || next.is_none()) {
        continue;
      }

      match c {
        '\n' => (),
        c if c.is_whitespace() => res.push(' '),
        c => res.push(c),
      }
    }

    if whitespaces_only && fragments.peek().is_some() {
      res.push(' ')
    }
  }

  res
}
