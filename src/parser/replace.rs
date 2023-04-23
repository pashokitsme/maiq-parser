use chrono::{DateTime, Datelike, Utc};
use include_dir::{include_dir, Dir};
use maiq_shared::{default::DefaultDay, utils::time, Group, Lesson};

lazy_static! {
  pub static ref REPLACEMENTS: Vec<DefaultDay> = load_defaults();
}

static DEFAULT_JSON_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/default/");

pub fn replace_all(groups: &mut [Group], date: DateTime<Utc>) {
  groups.iter_mut().for_each(|g| {
    g.lessons
      .iter_mut()
      .for_each(|l| try_replace_if_need(&g.name, l, date))
  });
}

pub fn try_replace_if_need(group_name: &str, lesson: &mut Lesson, date: DateTime<Utc>) {
  if matches!(lesson.name.as_str(), "По расписанию" | "по расписанию") {
    try_replace(lesson, group_name, date)
  }
}

pub fn try_replace(lesson: &mut Lesson, group_name: &str, date: DateTime<Utc>) {
  let weekday = date.weekday();
  println!("{} {:?}", date, weekday);
  let is_even = time::is_week_even(&date);
  if let Some(l) = REPLACEMENTS.iter().find(|d| d.day == weekday).and_then(|d| {
    d.groups.iter().find(|g| g.name.as_str() == group_name).and_then(|g| {
      g.lessons.iter().find(|l| {
        println!("{} {:?} {:?} {}", group_name, &l.num, &l.is_even, &l.name);
        match l.is_even {
          Some(e) => l.num == lesson.num && e == is_even,
          None => l.num == lesson.num,
        }
      })
    })
  }) {
    println!("**");
    *lesson = Lesson {
      num: lesson.num.clone(),
      name: l.name.clone(),
      subgroup: l.subgroup,
      teacher: l.teacher.clone(),
      classroom: l.classroom.clone(),
    }
  }
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
