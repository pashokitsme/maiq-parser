#[cfg(feature = "cli")]
mod cli {
  use maiq_parser::{compare::distinct, fetch_snapshot, warmup_defaults, Fetch};
  use maiq_shared::{Group, Snapshot};
  use std::{env, fs, io::BufWriter, process::exit};

  enum Command {
    Fetch(Fetch),
    Distinct,
    Dump(Fetch),
  }

  pub async fn run() {
    dotenvy::dotenv().ok();

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
      Command::Fetch(ref fetch) => match fetch_snapshot(fetch).await {
        Ok(snapshot) => match target_group {
          Some(g) => display_group(snapshot, &g),
          None => print_snapshot(&snapshot),
        },
        Err(err) => eprintln!("error -> {err}"),
      },
      Command::Distinct => show_distinct().await,
      Command::Dump(ref fetch) => do_dump(fetch).await,
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
    let today = fetch_snapshot(&Fetch::Today).await.ok();
    let other = fetch_snapshot(&Fetch::Next).await.ok();
    for group in distinct(other.as_ref(), other.as_ref()) {
      print!("{} ", group);
    }
  }

  async fn do_dump(fetch: &Fetch) {
    let snapshot = fetch_snapshot(fetch).await;
    if let Err(err) = snapshot {
      println!("error -> {}", err);
      return;
    }
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
