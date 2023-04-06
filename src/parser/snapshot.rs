use maiq_shared::{Group, Snapshot};

use crate::env;

use super::table::Table;

pub fn parse_snapshot(table: Table) -> Result<Snapshot, ()> {
  let groups = make_groups();
  let valid_name = |name: &str| {
    let name = name.split(' ').next().unwrap_or_default();
    groups.iter().any(|g| g.name == name)
  };
  let mut group_cursor: Option<String> = None;
  for mut row in table.values.iter().map(|vec| vec.iter()) {
    let ([group_name, subgroup], num) = {
      match row.next() {
        Some(x) if valid_name(x) => {
          if matches!(group_cursor, Some(ref c) if *c != *x) {
            group_cursor = Some(x.clone());
          }
          (split_group_name(Some(x)), row.next().map(|x| x.trim()))
        }
        Some(_) => (split_group_name(group_cursor.as_deref()), row.next().map(|x| x.trim())),
        None => continue,
      }
    };

    // let group_name = if let Some(g) = group_name { g } else { continue };
    let [name, teacher] = split_teacher(row.next().map(|x| &**x));
    let classroom = row.next();
    println!("{group_name:?} {subgroup:?} {classroom:?} {num:?} {name:?} {teacher:?}")
  }
  Err(())
}

fn split_teacher(raw: Option<&str>) -> [Option<&str>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };

  match raw.rsplit_once(',') {
    Some(x) => [Some(x.0.trim()), Some(x.1.trim())],
    None => [Some(raw), None],
  }
}

fn split_group_name(raw: Option<&str>) -> [Option<&str>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };
  let mut split = raw.split(' ').map(|x| x.trim());
  [split.next(), split.next()]
}

fn make_groups() -> Vec<Group> {
  let names: Vec<String> = env::groups().into();
  let mut groups = Vec::with_capacity(names.len());
  for name in names.into_iter() {
    groups.push(Group::new(name));
  }

  groups
}
