use std::{iter::Peekable, slice::Iter};

use chrono::{DateTime, Utc};
use log::warn;
use maiq_shared::{Group, Lesson, Snapshot};

use crate::env;

use super::{date, table::Table};

type GroupCursor = Option<String>;

#[derive(Debug, Clone, Default)]
enum Num {
  Actual(String),
  Previous,
  #[default]
  None,
}

#[derive(Debug, Default)]
struct RawLesson {
  num: Num,
  group_name: Option<String>,
  subgroup: Option<String>,
  name: Option<String>,
  teacher: Option<String>,
  classroom: Option<String>,
}

macro_rules! empty_to_none {
  ($e: expr) => {
    match $e {
      Some(x) if !x.is_empty() => Some(x),
      _ => None,
    }
  };
}

pub fn parse_snapshot(table: Table, fallback_date: DateTime<Utc>) -> anyhow::Result<Snapshot> {
  let mut rows = table.rows.into_iter();
  let date = date::parse_date(&mut rows).unwrap_or(fallback_date);
  let mut groups = make_groups();
  let mut group_cursor: GroupCursor = None;
  let is_name_valid = |name: &str| {
    let name = name.split(' ').next().unwrap_or_default();
    groups.iter().any(|g| g.name == name)
  };

  let mut lessons = rows
    .map(|vec| parse_row(&mut vec.iter().peekable(), &mut group_cursor, is_name_valid))
    .collect::<Vec<RawLesson>>();
  repair_nums(&mut lessons);
  assign_lessons_to_groups(lessons, &mut groups);
  groups.retain(|g| !g.lessons.is_empty());
  groups
    .iter_mut()
    .for_each(|g| g.lessons.sort_by(|a, b| a.num.cmp(&b.num)));

  Ok(Snapshot::new(groups, date))
}

fn assign_lessons_to_groups(lessons: Vec<RawLesson>, groups: &mut [Group]) {
  for lesson in lessons.into_iter() {
    if lesson.group_name.is_none() {
      continue;
    }
    let name = lesson.group_name.unwrap();
    let group = groups.iter_mut().find(|x| x.name == name);
    if group.is_none() {
      warn!("Attempt to add {name} to groups that doesn't exists");
      continue;
    }
    let group = group.unwrap();

    let nums = match lesson.num {
      Num::Actual(x) => x,
      _ => "Нет".into(),
    };

    for num in nums.split(',').map(|x| x.trim()) {
      group.lessons.push(Lesson {
        num: num.into(),
        name: lesson.name.clone().unwrap_or("?".into()),
        subgroup: lesson.subgroup.clone().and_then(|x| x.parse().ok()),
        teacher: lesson.teacher.clone(),
        classroom: lesson.classroom.clone(),
      })
    }
  }
}

fn parse_row<'a, F>(row: &mut Peekable<Iter<String>>, group_cursor: &mut GroupCursor, is_name_valid: F) -> RawLesson
where
  F: Fn(&str) -> bool + 'a,
{
  let ([group_name, subgroup], num) = {
    match row.next() {
      Some(x) if is_name_valid(x) => {
        if !matches!(group_cursor, Some(ref c) if *c == *x) {
          *group_cursor = Some(x.clone());
        }
        (split_group_name(Some(x)), parse_num(row))
      }
      Some(x) => (split_group_name(group_cursor.as_deref()), Num::Actual(x.clone())),
      _ => return RawLesson::default(),
    }
  };

  let [name, teacher] = split_teacher(row.next().map(|x| &**x));
  let classroom = empty_to_none!(row.next());

  RawLesson { num, group_name, subgroup, name, teacher, classroom: classroom.cloned() }
}

fn parse_num(row: &mut Peekable<Iter<String>>) -> Num {
  match row.peek().map(|x| is_num(x)).unwrap_or(false) {
    true => empty_to_none!(row.next().map(|x| x.trim().to_string()))
      .map(Num::Actual)
      .unwrap_or(Num::None),
    false => Num::Previous,
  }
}

fn repair_nums(lessons: &mut [RawLesson]) {
  let mut iter = lessons.iter_mut();
  let mut previous = iter.next().unwrap();
  for lesson in iter {
    if let Num::Previous = lesson.num {
      lesson.num = previous.num.clone();
    }
    previous = lesson;
  }
}

fn is_num(raw: &str) -> bool {
  const SKIP: [char; 6] = ['(', ')', ',', '.', 'ч', ' '];
  raw.chars().all(|c| SKIP.contains(&c) || c.is_numeric())
}

#[test]
fn __test_is_num() {
  assert!(is_num("1,2,3"));
  assert!(is_num("1,2,3(1ч)"));
  assert!(!is_num("Информационные технологии, Иванов И.Л."));
  assert!(is_num(""));
}

fn split_teacher(raw: Option<&str>) -> [Option<String>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };

  match raw.rsplit_once(',') {
    Some(x) => [Some(x.0.trim().into()), empty_to_none!(Some(x.1.trim().to_string()))],
    None => [empty_to_none!(Some(raw.to_string())), None],
  }
}

fn split_group_name(raw: Option<&str>) -> [Option<String>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };
  let mut split = raw.split(' ').map(|x| x.trim());
  [empty_to_none!(split.next().map(|x| x.to_string())), empty_to_none!(split.next().map(|x| x.to_string()))]
}

fn make_groups() -> Vec<Group> {
  let names: Vec<String> = env::groups().into();
  let mut groups = Vec::with_capacity(names.len());
  for name in names.into_iter() {
    groups.push(Group::new(name));
  }

  groups
}
