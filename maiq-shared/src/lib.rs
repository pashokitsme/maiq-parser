pub mod utils;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};
use utils::bytes_as_str;

use crate::utils::now;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
  pub name: String,
  pub lessons: Vec<Lesson>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lesson {
  pub num: usize,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subgroup: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub teacher: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub classroom: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
  pub date: DateTime<Utc>,
  pub is_week_even: bool,
  pub parsed_date: DateTime<Utc>,
  pub uid: String,
  pub groups: Vec<Group>,
}

impl Snapshot {
  /// `date` is `None` = today.
  pub fn new(groups: Vec<Group>, is_even: bool, date: DateTime<Utc>) -> Self {
    let now = chrono::Utc::now() + Duration::hours(3);
    let hash = Self::get_hash(&groups);
    Self { date, uid: hash, is_week_even: is_even, groups, parsed_date: now }
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

  fn get_hash(groups: &Vec<Group>) -> String {
    let mut hasher = Sha256::new();
    for group in groups {
      hasher.update(group.name.as_bytes());
      for lesson in &group.lessons {
        hasher.update(lesson.classroom.clone().unwrap_or_default().as_bytes());
        hasher.update(lesson.teacher.clone().unwrap_or_default().as_bytes());
        hasher.update(lesson.name.as_bytes());
        hasher.update(&num_as_bytes!(lesson.subgroup.unwrap_or(0), usize));
        hasher.update(&num_as_bytes!(lesson.num, usize));
      }
    }

    bytes_as_str(&hasher.finalize()[..])
  }
}
