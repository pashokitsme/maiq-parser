use std::fs;

use chrono::Datelike;
use maiq_shared::{default::DefaultDay, utils, Lesson};

lazy_static! {
  static ref FILE_NAMES: [&'static str; 6] = ["mon", "tue", "wed", "thu", "fri", "sat"];
  pub static ref REPLECEMENTS: Vec<DefaultDay> = load_defaults();
}

pub fn replace<'a>(num: usize, group_name: &'a str, is_even: bool, date_offset: i64) -> Option<Lesson> {
  let now = utils::now_date(date_offset).date_naive().weekday();
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
  FILE_NAMES
    .iter()
    .map(|f| read(&format!("default/{}.json", f)).expect(&format!("Can't load default for {}", f)))
    .collect::<Vec<DefaultDay>>()
}

fn read(path: &String) -> Option<DefaultDay> {
  fs::read_to_string(path).ok().map(|content| {
    serde_json::from_str(content.as_str()).expect(format!("Can't parse default timetable from `{}`", &path).as_str())
  })
}
