#![cfg(feature = "cli")]

use std::{env, process::exit};

use maiq_parser::{fetch_snapshot, warmup_defaults, Fetch};

use maiq_shared::{Group, Snapshot};

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  let mut args = env::args().skip(1);
  let mut fetch = None;
  let mut target_group = None;

  while let Some(arg) = args.next() {
    match &*arg {
      "today" | "t" => set_if_none(&mut fetch, Fetch::Today),
      "next" | "n" => set_if_none(&mut fetch, Fetch::Next),
      "--group" | "-g" => match args.next() {
        Some(group) => set_if_none(&mut target_group, group),
        None => usage_exit(),
      },
      "--help" | "-h" => usage_exit(),
      _ => (),
    }
  }

  if fetch.is_none() {
    usage_exit()
  }

  warmup_defaults();

  match fetch_snapshot(&fetch.unwrap()).await {
    Ok(snapshot) => match target_group {
      Some(g) => display_group(snapshot, &g),
      None => print_snapshot(&snapshot),
    },
    Err(err) => eprintln!("error -> {err}"),
  }
}

fn set_if_none<T>(param: &mut Option<T>, value: T) {
  match param {
    None => *param = Some(value),
    _ => usage_exit(),
  }
}

fn usage_exit() {
  println!(
    r#"usage: <fetch> <options>
    fetch: 
      today (t) | next (n) 
      
    options:
      --group (-g) <name> - print only group
      --help (-h) - this message"#
  );
  exit(0);
}

fn display_group(snapshot: Snapshot, group_name: &str) {
  println!("{} от {}\n", snapshot.uid, snapshot.date);
  let group = snapshot.group(group_name);
  match group {
    Some(g) => print_group(g),
    None => println!("Нет группы {}", group_name),
  }
}

fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.uid, s.date);
  for group in &s.groups {
    print_group(group);
    println!()
  }
}

fn print_group(g: &Group) {
  println!("Группа {} #{} ({})", g.name, g.uid, g.lessons.len());
  for lesson in &g.lessons {
    print!("\t#{} ", lesson.num);
    if let Some(sub) = lesson.subgroup {
      print!("(п. {}) ", sub)
    }
    print!("{} ", lesson.name);

    if let Some(classroom) = lesson.classroom.as_ref() {
      print!("в {}", classroom);
    }

    if let Some(teacher) = lesson.teacher.as_ref() {
      print!(". Преподаватель: {}", teacher)
    }
    println!()
  }
}
