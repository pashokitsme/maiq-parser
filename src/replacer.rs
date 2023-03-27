use chrono::{DateTime, Datelike, Utc};
use include_dir::{include_dir, Dir};
#[cfg(not(feature = "cli"))]
use log::warn;
use maiq_shared::{default::DefaultDay, utils::time, Lesson};

lazy_static! {
  pub static ref REPLECEMENTS: Vec<DefaultDay> = load_defaults();
}

static DEFAULT_JSON_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/default/");

pub fn replace_or_clone(num: u8, group_name: &str, lesson: &Lesson, date: DateTime<Utc>) -> Lesson {
  let mut lesson = match lesson.name.as_str() {
    "По расписанию" | "по расписанию" => replace(num, group_name, date).unwrap_or_else(|| lesson.clone()),
    _ => lesson.clone(),
  };
  lesson.num = num;
  lesson
}

pub fn replace(num: u8, group_name: &str, date: DateTime<Utc>) -> Option<Lesson> {
  let weekday = date.date_naive().weekday();
  let is_even = time::is_week_even(&date);
  REPLECEMENTS
    .iter()
    .find(|d| d.day == weekday)
    .and_then(|d| {
      d.groups.iter().find(|g| g.name.as_str() == group_name).and_then(|g| {
        g.lessons.iter().find(|l| match l.is_even {
          Some(e) => l.num == num && e == is_even,
          None => l.num == num,
        })
      })
    })
    .map(|l| {
      let l = l.clone();
      Lesson { num, name: l.name, subgroup: l.subgroup, teacher: l.teacher, classroom: l.classroom }
    })
}

fn load_defaults() -> Vec<DefaultDay> {
  DEFAULT_JSON_DIR
    .files()
    .filter(|file| matches!(file.path().extension(), Some(ext) if ext == "json"))
    .map(|file| {
      serde_json::from_str(file.contents_utf8().expect("Unable to read default file"))
        .unwrap_or_else(|_| panic!("Unable to parse {}", file.path().display()))
    })
    .collect::<Vec<DefaultDay>>()
}
