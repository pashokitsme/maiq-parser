use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use chrono::{DateTime, Utc};
use maiq_structs::{
  timetable::{Group, Lesson, Snapshot},
  utils,
};
use regex::Regex;
use scraper::Html;
use table_extract::Row;

use crate::{fetch::Fetched, ParserError};

#[derive(Clone)]
struct ParsedLesson {
  pub group: String,
  pub nums: Vec<usize>,
  pub lesson: Lesson,
}

lazy_static! {
  static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();
  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);
  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", "", ""];
}

// todo: rewrite table_extract with tl crate
// todo: parse row and then use it instead of parsing parts of it
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

  Ok(Snapshot::new(groups, date.1, date.0))
}

fn parse_date(row: &Row) -> (DateTime<Utc>, bool) {
  let full_str_binding = as_text(row.iter().next().unwrap());
  let mut iter = full_str_binding.trim().split(' ').rev();
  let even_or_not = match iter.next().unwrap() {
    "(числитель)" => false,
    "(знаменатель)" => true,
    // ? Is it really neseccery to beware of this?
    _ => false,
  };
  let weekday = iter.skip(1).next().unwrap();
  let today = utils::current_date(0);
  (utils::map_day(&today, weekday), even_or_not)
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
