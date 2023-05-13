use std::env;

use crate::Snapshot;
use log::debug;

lazy_static::lazy_static! {
  static ref GROUPS: Vec<String> = env::var("GROUPS").unwrap()
      .split(';')
      .map(|s| s.trim().to_owned())
      .collect::<Vec<String>>();
}

pub fn distinct(previous: Option<&Snapshot>, new: Option<&Snapshot>) -> Vec<String> {
  debug!("Comparing {:?} & {:?}", previous.map(|x| &x.uid), new.map(|x| &x.uid));
  let (previous, new) = match (previous, new) {
    (Some(l), Some(r)) if l.uid == r.uid => return vec![],
    (Some(l), Some(r)) => (l, r),
    (Some(_), None) => return vec![],
    (None, Some(_)) => return GROUPS.clone(),
    (None, None) => return vec![],
  };

  debug!(
    "Comparing groups:\n{:?}\nand\n{:?}",
    previous.groups.iter().map(|x| &x.name).collect::<Vec<&String>>(),
    new.groups.iter().map(|x| &x.name).collect::<Vec<&String>>()
  );

  let mut changes = GROUPS.clone();

  let is_updated = |name: &String| -> bool {
    let prev = previous.group(&*name);
    let new = new.group(&*name);

    let result = match (prev, new) {
      (None, Some(_)) => true,
      (Some(_), None) => true,
      (Some(p), Some(n)) if p.uid != n.uid => true,
      _ => false,
    };

    debug!(
      "Comparing {} {:?} & {:?}... {}",
      name,
      prev.map(|x| &x.uid),
      new.map(|x| &x.uid),
      if !result { "equals" } else { "distincts" }
    );

    result
  };

  changes.retain(is_updated);
  changes
}
