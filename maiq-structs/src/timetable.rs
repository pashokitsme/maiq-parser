use chrono::{DateTime, Duration, Utc};
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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subgroup: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub teacher: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub classroom: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
  // #[serde(deserialize_with = "from_ts")]
  pub date: DateTime<Utc>,
  // #[serde(deserialize_with = "from_ts")]
  pub parsed_date: DateTime<Utc>,
  pub uid: String,
  pub groups: Vec<Group>,
}

impl Snapshot {
  /// `date` is `None` = today.
  pub fn new(groups: Vec<Group>, date: DateTime<Utc>) -> Self {
    let now = chrono::Utc::now() + Duration::hours(3);
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
