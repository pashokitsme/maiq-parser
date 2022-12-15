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
  let mut prev: Option<Lesson> = None;
  for row in table.iter().skip(3) {
    let lesson = parse_lesson(&row, &prev);
    prev = lesson.clone();
    println!("{:?}", lesson);
  }
}

fn parse_lesson(row: &Row, prev: &Option<Lesson>) -> Option<Lesson> {
  let mut row = row.iter().peekable();
  if as_text(row.peek().unwrap()).is_empty() {
    return None;
  }

  if is_group(&row.peek().unwrap()) {
    row.next();
  }

  let nums_binding = as_text(&row.peek().unwrap());
  let mut nums = nums_binding.split(',');
  let (count, num) = (nums.clone().count(), nums.next().unwrap());

  let num = match num.parse::<usize>().ok() {
    Some(x) => {
      row.next();
      x
    }
    None => match prev {
      Some(x) => x.num,
      None => return None,
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

  Some(Lesson { num, count, name, teacher: teacher, classroom })
}

fn as_text(html: &str) -> String {
  let frag = Html::parse_fragment(html);
  CORASICK.replace_all(frag.root_element().text().collect::<String>().as_str(), CORASICK_REPLACE_PATTERNS.as_slice())
}

fn is_group(pattern: &str) -> bool {
  GROUP_REGEX.is_match(pattern)
}
