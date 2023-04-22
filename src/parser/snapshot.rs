use std::slice::Iter;

use log::warn;
use maiq_shared::{utils::time::now, Group, Lesson, Snapshot};

use crate::env;

use super::{date, table::Table};

type GroupCursor = Option<String>;

#[derive(Debug, Default)]
struct RawLesson {
  num: Option<String>,
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

pub fn parse_snapshot(table: Table) -> Result<Snapshot, ()> {
  let mut rows = table.rows.into_iter();
  let date = date::parse_date(&mut rows).unwrap_or(now());
  let mut groups = make_groups();
  let is_name_valid = |name: &str| {
    let name = name.split(' ').next().unwrap_or_default();
    groups.iter().any(|g| g.name == name)
  };
  let mut group_cursor: GroupCursor = None;
  let lessons = rows
    .map(|vec| parse_row(&mut vec.iter(), &mut group_cursor, is_name_valid))
    .collect::<Vec<RawLesson>>();

  insert(lessons, &mut groups);
  Ok(Snapshot::new(groups, date))
}

fn insert(lessons: Vec<RawLesson>, groups: &mut Vec<Group>) {
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
    for num in lesson.num.as_deref().unwrap_or("Нет").split(',').map(|x| x.trim()) {
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

fn parse_row<'a, F>(row: &mut Iter<String>, group_cursor: &mut GroupCursor, is_name_valid: F) -> RawLesson
where
  F: Fn(&str) -> bool + 'a,
{
  let ([group_name, subgroup], num) = {
    match row.next() {
      Some(x) if is_name_valid(x) => {
        if !matches!(group_cursor, Some(ref c) if *c == *x) {
          *group_cursor = Some(x.clone());
        }
        (split_group_name(Some(x)), empty_to_none!(row.next().map(|x| x.trim())))
      }
      Some(x) => (split_group_name(group_cursor.as_deref()), Some(x.as_str())),
      None => return RawLesson::default(),
    }
  };

  let [name, teacher] = split_teacher(row.next().map(|x| &**x));
  let classroom = empty_to_none!(row.next());

  RawLesson { num: num.map(|x| x.to_string()), group_name, subgroup, name, teacher, classroom: classroom.cloned() }
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
