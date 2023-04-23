pub mod compare;
pub mod default;
pub mod utils;

use std::fmt::Display;

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};
use utils::{bytes_as_str, time};

pub trait Uid {
  fn uid(&self) -> String {
    bytes_as_str(&self.uid_bytes())
  }

  fn uid_bytes(&self) -> [u8; 32];
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone)]
pub enum Num {
  Actual(String),
  Previous,
  #[default]
  None,
}

impl Num {
  pub fn is_some(&self) -> bool {
    matches!(self, Num::Actual(_))
  }
}

impl Display for Num {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Num::Actual(x) => f.write_str(&x),
      Num::Previous => f.write_str("?"),
      Num::None => f.write_str("Нет"),
    }
  }
}

impl Uid for Num {
  fn uid_bytes(&self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut res = [0u8; 32];

    match self {
      Num::Actual(x) => hasher.update(&x),
      _ => hasher.update([0]),
    }
    hasher.finalize_into((&mut res).into());
    res
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
  pub uid: String,
  pub name: String,
  pub lessons: Vec<Lesson>,
}

impl Group {
  pub fn new(name: String) -> Self {
    let mut g = Self { uid: String::with_capacity(10), name, lessons: vec![] };
    g.uid = g.uid();
    g
  }
}

impl Uid for Group {
  fn uid_bytes(&self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut res = [0u8; 32];
    hasher.update(&self.name);
    self.lessons.iter().for_each(|l| hasher.update(l.uid_bytes()));
    hasher.finalize_into((&mut res).into());
    res
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lesson {
  pub num: Num,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subgroup: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub teacher: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub classroom: Option<String>,
}

impl Uid for Lesson {
  fn uid_bytes<'a>(&'a self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut res = [0u8; 32];
    hasher.update(self.classroom.clone().unwrap_or_default().as_bytes());
    hasher.update(self.teacher.clone().unwrap_or_default().as_bytes());
    hasher.update(self.name.as_bytes());
    hasher.update(&num_as_bytes!(self.subgroup.unwrap_or(0), u8));
    hasher.update(self.num.uid_bytes());
    hasher.finalize_into((&mut res).into());
    res
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
  pub date: DateTime<Utc>,
  pub parsed_date: DateTime<Utc>,
  pub uid: String,
  pub groups: Vec<Group>,
}

impl Snapshot {
  pub fn new(groups: Vec<Group>, date: DateTime<Utc>) -> Self {
    let now = chrono::Utc::now() + Duration::hours(3);
    let mut s = Self { date, uid: String::with_capacity(10), groups, parsed_date: now };
    s.uid = s.uid();
    s
  }

  pub fn group(&self, name: &str) -> Option<&Group> {
    self.groups.iter().find(|g| g.name.as_str() == name)
  }

  pub fn group_cloned(&self, name: &str) -> Option<Group> {
    self.group(name).cloned()
  }

  pub fn age(&self) -> Duration {
    time::now() - self.parsed_date
  }

  pub fn is_even(&self) -> bool {
    self.date.iso_week().week() % 2 != 0
  }

  pub fn tiny(&self, group: &str) -> TinySnapshot {
    let group = self
      .groups
      .iter()
      .find(|g| g.name == group)
      .and_then(|g| Some(g.to_owned()));

    TinySnapshot { uid: self.uid.clone(), date: self.date, parsed_date: self.parsed_date, group }
  }
}

impl Uid for Snapshot {
  fn uid_bytes(&self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut res = [0u8; 32];
    self.groups.iter().for_each(|g| hasher.update(&g.uid_bytes()));
    hasher.finalize_into((&mut res).into());
    res
  }
}

#[derive(Debug, Serialize, Clone)]
pub struct TinySnapshot {
  pub uid: String,
  pub date: DateTime<Utc>,
  pub parsed_date: DateTime<Utc>,
  pub group: Option<Group>,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Fetch {
  Today,
  Next,
}

pub trait FetchUrl {
  fn url(&self) -> &'static str;
  fn date(&self) -> DateTime<Utc>;
}

impl FetchUrl for Fetch {
  fn url(&self) -> &'static str {
    match self {
      Fetch::Today => "https://rsp.chemk.org/4korp/today.htm",
      Fetch::Next => "https://rsp.chemk.org/4korp/tomorrow.htm",
    }
  }

  fn date(&self) -> DateTime<Utc> {
    let now = time::now_date();
    match self {
      Fetch::Today => match now.weekday() {
        chrono::Weekday::Sun => time::now_date_offset(-1),
        _ => now,
      },
      Fetch::Next => match now.weekday() {
        chrono::Weekday::Sat => time::now_date_offset(2),
        _ => time::now_date_offset(1),
      },
    }
  }
}
