use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use chrono::{DateTime, Utc};
use regex::Regex;
use scraper::Html;
use table_extract::Row;

use crate::{
  fetch::Fetched,
  timetable::{Group, Lesson, Snapshot},
  utils::{self, map_day},
  ParserError,
};

#[derive(Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<usize>,
  pub lesson: Lesson,
}

lazy_static::lazy_static! {
  static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();

  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);

  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", "", ""];
}

// todo: use tl crate instead table_extract or rewrite it?
// todo: parse row and then use it instead parse parts of it
pub async fn parse(fetched: &Fetched) -> Result<Snapshot, ParserError> {
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

  let groups = map_lessons_to_groups(&lessons);

  Ok(Snapshot::new(groups, Some(date)))
}

fn parse_date(row: &Row) -> DateTime<Utc> {
  let full_str = as_text(row.iter().next().unwrap());
  let weekday = full_str.trim().split(' ').rev().skip(2).next().unwrap();
  let today = utils::current_date();
  map_day(&today, weekday)
}

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
      Some(x) => x.trim().parse::<usize>().ok(),
      None => None,
    };
    (group.to_string(), subgroup.clone())
  } else {
    let cloned = prev.clone().unwrap();
    (cloned.group, cloned.lesson.subgroup)
  };

  let nums_binding = as_text(&row.next().unwrap());

  let mut nums: Vec<usize> = vec![];
  for num in nums_binding.split(',') {
    let num = match num.parse::<usize>().ok() {
      Some(x) => x,
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
        "" => None,
        x => Some(x.to_string()),
      },
      None => None,
    },
  };

  let mut name_n_teacher = name_n_teacher.split(',').map(|x| x.trim().to_string());
  let name = name_n_teacher.next().unwrap();
  let teacher = name_n_teacher.next();

  Ok(Some(ParsedLesson { group, nums, lesson: Lesson { num: 0, name, subgroup, teacher, classroom } }))
}

fn map_lessons_to_groups(vec: &Vec<ParsedLesson>) -> Vec<Group> {
  let mut res: Vec<Group> = vec![];
  for lesson in vec {
    for num in &lesson.nums {
      if *num < 1 {
        continue;
      }

      let name = lesson.group.as_str().clone();
      let group = if let Some(x) = res.iter().position(|x| x.name.as_str() == name) {
        &mut res[x]
      } else {
        res.push(Group { name: name.to_string(), lessons: vec![] });
        res.last_mut().unwrap()
      };

      let mut lesson = lesson.lesson.clone();
      lesson.num = *num;

      group.lessons.push(lesson)
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
