use std::fs;

use chrono::Datelike;
use maiq_shared::{default::DefaultDay, utils, Lesson};

lazy_static! {
  pub static ref REPLECEMENTS: Vec<DefaultDay> = load_defaults();
}

pub fn replace_or_clone(num: u8, group_name: &str, lesson: &Lesson, is_even: bool, offset_days: i64) -> Lesson {
  let mut lesson = match lesson.name.as_str() {
    "По расписанию" => replaced(num, group_name, is_even, offset_days).unwrap_or_else(|| lesson.clone()),
    _ => lesson.clone(),
  };
  lesson.num = num;
  lesson
}

pub fn replaced(num: u8, group_name: &str, is_even: bool, offset_days: i64) -> Option<Lesson> {
  let now = utils::now_date(offset_days).date_naive().weekday();
  REPLECEMENTS
    .iter()
    .find(|d| d.day == now)
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
  ["mon", "tue", "wed", "thu", "fri", "sat"]
    .iter()
    .map(|f| read(&format!("default/{}.json", f)).expect(&format!("Can't load default for {}", f)))
    .collect::<Vec<DefaultDay>>()
}

fn read(path: &String) -> Option<DefaultDay> {
  fs::read_to_string(path).ok().map(|content| {
    serde_json::from_str(content.as_str()).expect(format!("Can't parse default timetable from `{}`", &path).as_str())
  })
}
