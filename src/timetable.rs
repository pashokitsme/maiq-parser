use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{digest::Digest, Sha256};

use crate::utils;

#[derive(Debug, Serialize)]
pub struct Group {
  pub name: String,
  pub lessons: Vec<Lesson>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Lesson {
  pub num: usize,
  pub count: usize,
  pub name: String,
  pub teacher: Option<String>,
  pub classroom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Day {
  pub date: DateTime<Utc>,
  pub hash: String,
  pub groups: Vec<Group>,
}

impl Day {
  //? date is None = today.
  pub fn new(groups: Vec<Group>, date: Option<DateTime<Utc>>) -> Self {
    let date = date.unwrap_or(chrono::Utc::now());
    let hash = Self::get_hash(&groups);
    Self { date, hash, groups }
  }

  fn get_hash(groups: &Vec<Group>) -> String {
    let mut hasher = Sha256::new();
    for group in groups {
      hasher.update(&group.name);
      for lesson in &group.lessons {
        hasher.update(&lesson.classroom.clone().unwrap_or_default());
        hasher.update(&lesson.teacher.clone().unwrap_or_default());
        hasher.update(&*lesson.name);
        hasher.update(utils::usize_as_bytes(lesson.num));
        hasher.update(utils::usize_as_bytes(lesson.count));
      }
    }

    pretty_sha2::sha512::to_str(&hasher.finalize()[..])
  }
}