use crate::Snapshot;

lazy_static::lazy_static! {
  static ref GROUPS: Vec<String> = "Ит1-22 Са1-21 Са3-21 С1-21 С3-21 Ир1-21 Ир3-21 Ир5-21 С1-20 С3-20 Ип1-20 Ип3-20 Ир1-20 Ир3-20 Ир5-20 Кс1-20 Кс3-20 Кс5-20 С1-19 С3-19 С1-18 С3-18"
      .split_whitespace()
      .map(|s| s.to_owned())
      .collect::<Vec<String>>();
}

pub fn distinct(previous: Option<&Snapshot>, new: Option<&Snapshot>) -> Vec<String> {
  let mut groups_to_update = GROUPS.clone();

  let (previous, new) = match (previous, new) {
    (Some(l), Some(r)) if l.uid == r.uid => return vec![],
    (Some(l), Some(r)) => (l, r),
    (Some(_), None) => return vec![],
    (None, Some(_)) => return groups_to_update,
    (None, None) => return vec![],
  };

  let updated = |name: &String| -> bool {
    let prev = previous.group(&*name);
    let new = new.group(&*name);

    match (prev, new) {
      (None, Some(_)) => true,
      (Some(_), None) => true,
      (Some(p), Some(n)) if n.uid != p.uid => true,
      _ => false,
    }
  };

  groups_to_update.retain(updated);
  groups_to_update
}
