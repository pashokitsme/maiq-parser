use std::env;

use crate::Snapshot;

lazy_static::lazy_static! {
  static ref GROUPS: Vec<String> = env::var("GROUPS").unwrap()
      .split(';')
      .map(|s| s.trim().to_owned())
      .collect::<Vec<String>>();
}

pub fn distinct(previous: Option<&Snapshot>, new: Option<&Snapshot>) -> Vec<String> {
  let (previous, new) = match (previous, new) {
    (Some(l), Some(r)) if l.uid == r.uid => return vec![],
    (Some(l), Some(r)) => (l, r),
    (Some(_), None) => return vec![],
    (None, Some(_)) => return GROUPS.clone(),
    (None, None) => return vec![],
  };

  let mut changes = GROUPS.clone();

  let is_updated = |name: &String| -> bool {
    let prev = previous.group(&*name);
    let new = new.group(&*name);

    match (prev, new) {
      (None, Some(_)) => true,
      (Some(_), None) => true,
      (Some(p), Some(n)) if n.uid != p.uid => true,
      _ => false,
    }
  };

  changes.retain(is_updated);
  changes
}
