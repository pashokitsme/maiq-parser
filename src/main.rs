use maiq_parser::{
  fetch::{fetch, Fetch},
  parser::parse,
  timetable::Snapshot,
};

// ? Too lazy to write tests

#[allow(dead_code, unused_variables)]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  let fetched = fetch(Fetch::Tomorrow).await.unwrap();
  println!("{}", fetched);
  let s = match parse(&fetched).await {
    Ok(x) => x,
    Err(x) => return println!("Ошибка: {}", x),
  };

  // print_snapshot(&s)
  println!("Date: {}", s.date)
}

#[allow(dead_code)]
fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.uid, s.date);
  for group in &s.groups {
    println!("Группа {}", group.name);
    for lesson in &group.lessons {
      print!("\t#{} ", lesson.num);
      if let Some(sub) = lesson.subgroup {
        print!(" (п. {}) ", sub)
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
    println!()
  }
}
