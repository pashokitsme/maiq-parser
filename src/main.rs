#[cfg(feature = "cli")]
mod cli {
  use colored::Colorize;
  use maiq_parser::{compare::distinct, snapshot_from_remote, warmup_defaults, Fetch, Num};
  use maiq_shared::{Group, Snapshot};
  use std::{env, fs, io::BufWriter, process::exit};

  enum Command {
    Fetch(Fetch),
    Distinct,
    Dump(Fetch),
  }

  pub async fn run() {
    dotenvy::dotenv().ok();
    maiq_parser::env::init();
    pretty_env_logger::init();

    let mut args = env::args().skip(1);
    let mut command = None;
    let mut target_group = None;

    while let Some(arg) = args.next() {
      match &*arg {
        "today" | "t" => set_if_none(&mut command, Command::Fetch(Fetch::Today)),
        "next" | "n" => set_if_none(&mut command, Command::Fetch(Fetch::Next)),
        "distinct" | "dt" => set_if_none(&mut command, Command::Distinct),
        "dump-today" => set_if_none(&mut command, Command::Dump(Fetch::Today)),
        "dump-next" => set_if_none(&mut command, Command::Dump(Fetch::Next)),
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
      Command::Fetch(ref fetch) => match snapshot_from_remote(fetch).await {
        Ok(snapshot) => match target_group {
          Some(g) => display_group(snapshot, &g),
          None => print_snapshot(&snapshot),
        },
        Err(x) => eprintln!("error -> {}", x),
      },
      Command::Distinct => show_distinct().await,
      Command::Dump(ref fetch) => dump(fetch).await,
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
      dump-today | dump-next
    options:
      --group (-g) <name> - вывести только указанную группу
      --help (-h) - это сообщение"#
    );
    exit(0);
  }

  async fn show_distinct() {
    let today = snapshot_from_remote(&Fetch::Today).await.ok();
    let other = snapshot_from_remote(&Fetch::Next).await.ok();
    for group in distinct(today.as_ref(), other.as_ref()) {
      print!("{} ", group);
    }
  }

  async fn dump(fetch: &Fetch) {
    let snapshot = snapshot_from_remote(fetch).await;
    let snapshot = snapshot.unwrap();
    let file_name = format!("{}_{}.json", snapshot.date.format("%d-%m-%Y"), snapshot.uid);
    let file = fs::File::create(file_name).expect("unable to open file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &snapshot).expect("unable to write file");
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
    println!("Группа {} ({}) ({})", g.name.bright_white(), g.uid.purple(), g.lessons.len());
    for lesson in &g.lessons {
      print!("\t");
      if let Num::Actual(ref num) = lesson.num {
        print!("{} ", format!("#{}", num).bright_white());
      }
      if let Some(sub) = lesson.subgroup {
        print!("{} ", format!("(п. {sub})").green())
      }
      print!("{} ", lesson.name);

      if let Some(classroom) = lesson.classroom.as_ref() {
        print!("в {}", classroom.green());
      }

      if let Some(teacher) = lesson.teacher.as_ref() {
        print!(". Преподаватель: {}", teacher.green())
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
