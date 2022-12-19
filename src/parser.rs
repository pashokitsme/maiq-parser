use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use regex::Regex;
use scraper::Html;
use table_extract::Row;

use crate::{
  fetch::Fetched,
  timetable::{Day, Group, Lesson},
  ParserError,
};

#[derive(Clone)]
struct ParsedLesson {
  pub group: String,
  pub lesson: Lesson,
}

lazy_static::lazy_static! {
  static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();

  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);

  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", "", ""];
}

//todo: use tl crate instead table_extract or rewrite it?
pub async fn parse(fetched: &Fetched) -> Result<Day, ParserError> {
  let table = match table_extract::Table::find_first(&fetched.html) {
    Some(x) => x,
    None => return Err(ParserError::NotYet),
  };
  let mut lessons = vec![];
  let mut prev: Option<ParsedLesson> = None;
  for row in table.iter().skip(3) {
    let lesson = parse_lesson(&row, &prev)?;
    if lesson.is_some() {
      prev = lesson.clone();
      lessons.push(lesson.unwrap());
    }
  }

  let groups = map_lessons_to_groups(&lessons);
  Ok(Day::new(groups, None))
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

  let nums_binding = as_text(&row.peek().unwrap());
  let mut nums = nums_binding.split(',');
  let (count, num) = (nums.clone().count(), nums.next().unwrap());

  let num = match num.parse::<usize>().ok() {
    Some(x) => {
      row.next();
      x
    }
    None => match prev {
      Some(x) => x.lesson.num,
      None => return Ok(None),
    },
  };

  let name_n_teacher = as_text(row.next().unwrap());
  let classroom = match name_n_teacher.as_str() {
    "Нет" => None,
    _ => match row.next() {
      Some(x) => Some(as_text(&x.as_str())),
      None => None,
    },
  };

  let mut name_n_teacher = name_n_teacher.split(',').map(|x| x.trim().to_string());
  let name = name_n_teacher.next().unwrap();
  let teacher = name_n_teacher.next();

  Ok(Some(ParsedLesson { group, lesson: Lesson { num, count, name, subgroup, teacher, classroom } }))
}

fn map_lessons_to_groups(vec: &Vec<ParsedLesson>) -> Vec<Group> {
  let mut res: Vec<Group> = vec![];
  for lesson in vec {
    let name = lesson.group.as_str().clone();
    let group = if let Some(x) = res.iter().position(|x| x.name.as_str() == name) {
      &mut res[x]
    } else {
      res.push(Group { name: name.to_string(), lessons: vec![] });
      res.last_mut().unwrap()
    };

    group.lessons.push(lesson.lesson.clone())
  }

  res
}

fn as_text(html: &str) -> String {
  let frag = Html::parse_fragment(html);
  CORASICK.replace_all(frag.root_element().text().collect::<String>().as_str(), CORASICK_REPLACE_PATTERNS.as_slice())
}

fn is_group(pattern: &str) -> bool {
  GROUP_REGEX.is_match(pattern)
}
