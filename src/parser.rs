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
    let row = parse_row(&row);
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

fn parse_lesson<'a>(row: &Vec<Option<String>>, prev: &Option<ParsedLesson>) -> Result<Option<ParsedLesson>, ParserError> {
  // println!("{:#?}", row);
  Ok(None)
}

fn parse_row<'a>(row: &Row) -> Vec<Option<String>> {
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
    r[4] = name_n_teacher.clone().last().and_then(|x| Some(x.trim().to_owned())); // teacher
    let take = name_n_teacher.clone().count() - 1;
    let name = name_n_teacher
      .take(take)
      .map(|x| x.trim())
      .collect::<Vec<&str>>()
      .join(", ");
    r[3] = Some(name); // name
  }

  r[5] = raw.next().and_then(|x| Some(x.trim().to_owned())); // classroom

  dbg!(&r);
  r
}

// Idk how it works :(
/*
fn parse_lesson(row: &Row, prev: &Option<ParsedLesson>) -> Result<Option<ParsedLesson>, ParserError> {
  let mut row = row.iter().peekable();
  println!("\n{}", row.clone().map(|x| format!("{} ", as_text(x))).collect::<String>());
  if as_text(row.peek().unwrap()).is_empty() {
    return Ok(None);
  }

  // * good
  let (group, subgroup) = if is_group(&as_text(row.peek().unwrap())) {
    let group_n_subgroup = as_text(row.next().unwrap());
    let mut group_n_subgroup = group_n_subgroup.split(&[' ', ' ', '\n']);
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

  println!("G {} SG {:?}", group, subgroup);

  let nums_binding = as_text(&row.peek().unwrap());
  let nums_str = nums_binding.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
  println!("NUMS {:?}", nums_str);

  let mut nums: Vec<u8> = vec![];

  if nums_str.len() == 0 || nums_str[0].parse::<u8>().is_err() {
    println!("CANT PARSE, prev: {:?}", prev);
    nums.push(match prev {
      Some(x) => x.lesson.num,
      None => {
        println!("PREV NONE");
        return Ok(None);
      }
    });
  } else {
    for num in nums_str {
      let num = match num.parse::<u8>().ok() {
        Some(x) => x,
        None => match prev {
          Some(x) => x.lesson.num,
          None => {
            println!("PREV NONE");
            return Ok(None);
          }
        },
      };
      nums.push(num);
    }
  }

  let name_n_teacher = as_text(row.next().unwrap());
  let classroom = match name_n_teacher.as_str() {
    "Нет" => None,
    _ => match row.next() {
      Some(x) => match as_text(&x.as_str()).as_str() {
        "" | " " | " " => None,
        x => Some(x.trim().to_string()),
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

*/

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
