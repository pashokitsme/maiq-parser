use chrono::{DateTime, Utc};
use sha2::{digest::Digest, Sha512};

pub struct Group {
  pub name: String,
  pub lessons: Vec<Lesson>,
}

pub struct Lesson {
  pub num: usize,
  pub name: String,
  pub classroom: String,
}

pub struct Day {
  pub date: DateTime<Utc>,
  pub hash: String,
  pub groups: Vec<Group>,
}

impl Day {
  pub fn new(date: Option<DateTime<Utc>>, groups: Vec<Group>) -> Day {
    let date = date.unwrap_or(chrono::Utc::now());
    let hash = Day::get_hash(&groups);
    Day { date, hash, groups }
  }

  fn get_hash(groups: &Vec<Group>) -> String {
    fn to_bytes<'a>(n: usize) -> [u8; 8] {
      let mut res = [0u8; (usize::BITS / 8) as usize];
      let mut n = n;
      for i in 0..(usize::BITS / 8) as usize {
        res[i] = ((n >> (8 * i)) & 0xff) as u8;
        n = n >> i;
      }
      res
    }

    let mut hasher = Sha512::new();
    for group in groups {
      hasher.update(&group.name);
      for lesson in &group.lessons {
        hasher.update(&lesson.classroom);
        hasher.update(&lesson.name);
        hasher.update(to_bytes(lesson.num));
      }
    }

    pretty_sha2::sha512::to_str(&hasher.finalize()[..])
  }
}
