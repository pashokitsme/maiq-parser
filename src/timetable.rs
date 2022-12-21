use chrono::{serde::ts_seconds::deserialize as from_ts, serde::ts_seconds::serialize as to_ts, DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};

use crate::utils::usize_as_bytes;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
  pub name: String,
  pub lessons: Vec<Lesson>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lesson {
  pub num: usize,
  pub name: String,
  pub subgroup: Option<usize>,
  pub teacher: Option<String>,
  pub classroom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snapshot {
  pub date: NaiveDate,
  #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
  pub parsed_date: DateTime<Utc>,
  pub uid: String,
  pub groups: Vec<Group>,
}

impl Snapshot {
  //? date is None = today.
  pub fn new(groups: Vec<Group>, date: Option<NaiveDate>) -> Self {
    let now = chrono::Utc::now();
    let date = date.unwrap_or(now.date_naive());
    let hash = Self::get_hash(&groups);
    Self { date, uid: hash, groups, parsed_date: now }
  }

  fn get_hash(groups: &Vec<Group>) -> String {
    let mut hasher = Sha256::new();
    for group in groups {
      hasher.update(&group.name);
      for lesson in &group.lessons {
        hasher.update(&lesson.classroom.clone().unwrap_or_default());
        hasher.update(&lesson.teacher.clone().unwrap_or_default());
        hasher.update(&lesson.name);
        hasher.update(usize_as_bytes(lesson.subgroup.unwrap_or(0)));
        hasher.update(usize_as_bytes(lesson.num));
      }
    }

    pretty_sha2::sha512::to_str(&hasher.finalize()[..])
  }
}
