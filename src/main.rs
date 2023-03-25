#[cfg(feature = "cli")]
mod cli {
  use maiq_parser::{compare::distinct, fetch_snapshot, warmup_defaults, Fetch};
  use maiq_shared::{Group, Snapshot};
  use std::{env, process::exit};

  enum Command {
    Fetch(Fetch),
    Distinct,
  }

  pub async fn run() {
    dotenvy::dotenv().ok();

    let mut args = env::args().skip(1);
    let mut command = None;
    let mut target_group = None;

    while let Some(arg) = args.next() {
      match &*arg {
        "today" | "t" => set_if_none(&mut command, Command::Fetch(Fetch::Today)),
        "next" | "n" => set_if_none(&mut command, Command::Fetch(Fetch::Today)),
        "distinct" | "dt" => set_if_none(&mut command, Command::Distinct),
        "--group" | "-g" => match args.next() {
          Some(group) => set_if_none(&mut target_group, group),
          None => usage_exit(),
        },
        "--help" | "-h" => usage_exit(),
        _ => (),
      }
    }

    if command.is_none() {
      usage_exit()
    }

    warmup_defaults();

    match command.unwrap() {
      Command::Fetch(ref fetch) => match fetch_snapshot(fetch).await {
        Ok(snapshot) => match target_group {
          Some(g) => display_group(snapshot, &g),
          None => print_snapshot(&snapshot),
        },
        Err(err) => eprintln!("error -> {err}"),
      },
      Command::Distinct => show_distinct().await,
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
      r#"использование: <команда> <параметры>
    команда: 
      today (t) | next (n) 
      distinct (dt)
    options:
      --group (-g) <name> - вывести только указанную группу
      --help (-h) - это сообщение"#
    );
    exit(0);
  }

  async fn show_distinct() {
    let today = fetch_snapshot(&Fetch::Today).await.ok();
    let other = fetch_snapshot(&Fetch::Next).await.ok();
    for group in distinct(today.as_ref(), other.as_ref()) {
      print!("{} ", group);
    }
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
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
  cli::run().await;
}

#[cfg(not(feature = "cli"))]
fn main() {}
