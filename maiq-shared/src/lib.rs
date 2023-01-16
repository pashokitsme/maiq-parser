pub mod default;
pub mod utils;

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};
use utils::bytes_as_str;

use crate::utils::now;

pub trait Uid {
  fn uid(&self) -> String {
    bytes_as_str(&self.uid_bytes())
  }

  fn uid_bytes(&self) -> [u8; 32];
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
  pub num: u8,
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
    hasher.update(&num_as_bytes!(self.num, u8));
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

  pub fn group<'n, 'g>(&'g self, name: &'n str) -> Option<&'g Group> {
    self.groups.iter().find(|g| g.name.as_str() == name)
  }

  pub fn group_cloned<'n>(&self, name: &'n str) -> Option<Group> {
    self.group(name).cloned()
  }

  pub fn age(&self) -> Duration {
    now(0) - self.parsed_date
  }

  pub fn is_even(&self) -> bool {
    self.date.iso_week().week() % 2 != 0
  }

  pub fn tiny<'a>(&self, group: &'a str) -> TinySnapshot {
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Fetch {
  Today,
  Next,
}

pub trait FetchUrl {
  fn url(&self) -> &'static str;
}

impl FetchUrl for Fetch {
  fn url(&self) -> &'static str {
    match self {
      Fetch::Today => "https://rsp.chemk.org/4korp/today.htm",
      Fetch::Next => "https://rsp.chemk.org/4korp/tomorrow.htm",
    }
  }
}
