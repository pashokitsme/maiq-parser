use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use regex::Regex;
use scraper::Html;
use std::fmt::Display;
use std::rc::Rc;
use std::time::Duration;
use stopwatch::Stopwatch;
use table_extract::Row;

use crate::timetable::Lesson;

#[derive(Debug)]
pub enum Fetch {
  Today,
  Tomorrow,
}

impl Fetch {
  pub fn url(&self) -> &'static str {
    match self {
      Fetch::Tomorrow => "https://rsp.chemk.org/4korp/tomorrow.htm",
      Fetch::Today => "https://rsp.chemk.org/4korp/today.htm",
    }
  }
}

pub struct Fetched {
  pub html: String,
  pub took: Duration,
  pub etag: String,
  pub fetch_mode: Fetch,
}

impl Display for Fetched {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "Get {:?}. HTML: (...{}), etag: {}, took {}ms",
      self.fetch_mode,
      self.html.len(),
      self.etag,
      self.took.as_millis()
    ))
  }
}

pub async fn fetch<'a>(fetch_mode: Fetch) -> Result<Fetched, reqwest::Error> {
  let mut watch = Stopwatch::start_new();
  let res = reqwest::get(fetch_mode.url()).await?;
  let etag = res.headers().get("ETag").unwrap().to_str().unwrap().replace("\"", "");
  let html = res.text_with_charset("windows-1251").await?;
  watch.stop();
  Ok(Fetched { html, took: watch.elapsed(), etag, fetch_mode })
}

lazy_static::lazy_static! {
  static ref GROUP_REGEX: Regex = Regex::new(r#"[А-я]{1,2}\d-\d{2}"#).unwrap();

  static ref CORASICK: AhoCorasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);

  static ref CORASICK_REPLACE_PATTERNS: [&'static str; 3] = [" ", "", ""];
}

//todo: use tl crate?
pub fn parse(fetched: Fetched) {
  let table = table_extract::Table::find_first(&fetched.html).unwrap();
  for row in table.iter().skip(3) {
    let lesson = parse_lesson(&row);
    println!("{:#?}", lesson);
  }
}

// #[derive(Debug)]
// struct ParsedLesson {
//   pub offset: usize,
//   pub count: usize,
//   pub lesson: Lesson,
// }

fn parse_lesson(row: &Row) -> Vec<Lesson> {
  let mut row = row.iter().peekable();
  let mut res = vec![];
  if text(row.peek().unwrap()).is_empty() {
    return res;
  }

  if is_group(&row.peek().unwrap()) {
    row.next();
  }

  let nums = text(row.next().unwrap())
    .split(',')
    .map(|x| {
      let x = x.trim();
      x.parse::<usize>().unwrap()
    })
    .collect::<Vec<usize>>();

  let name_n_teacher = text(row.next().unwrap());
  let classroom = match name_n_teacher.as_str() {
    "Нет" => Rc::new(None),
    _ => match row.next() {
      Some(x) => Rc::new(Some(text(&x.as_str()))),
      None => Rc::new(None),
    },
  };

  let mut name_n_teacher = name_n_teacher.split(',').map(|x| x.trim().to_string());
  let name = Rc::new(name_n_teacher.next().unwrap());
  let teacher = Rc::new(name_n_teacher.next());

  for &num in nums.iter() {
    res.push(Lesson { num, name: Rc::clone(&name), teacher: Rc::clone(&teacher), classroom: Rc::clone(&classroom) });
  }

  res
}

fn text(html: &str) -> String {
  let frag = Html::parse_fragment(html);
  CORASICK.replace_all(frag.root_element().text().collect::<String>().as_str(), CORASICK_REPLACE_PATTERNS.as_slice())
}

fn is_group(pattern: &str) -> bool {
  GROUP_REGEX.is_match(pattern)
}

// fn is_matches_group(source: &String) -> bool {
//   source.chars().rev().take(2).all(|c| c >= '0' && c <= '9')
//     && source.chars().rev().nth(2).unwrap_or_default() == '-'
//     && source
//       .chars()
//       .rev()
//       .skip(4)
//       .all(|c| (c >= 'а' && c <= 'я') || (c >= 'А' || c <= 'Я'))
// }
