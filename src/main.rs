use chrono::{DateTime, Days, Utc};
use maiq_parser::timetable::Snapshot;

// ? Too lazy to write tests

#[allow(dead_code, unused_variables)]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  let date = Utc::now()
    .date_naive()
    .checked_add_days(Days::new(1))
    .unwrap()
    .and_hms_opt(0, 0, 0)
    .unwrap();
  let utc = DateTime::<Utc>::from_utc(date, Utc);
  let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
  assert_eq!(date.timestamp(), utc.timestamp())
  // let fetched = fetch(Fetch::Tomorrow).await.unwrap();
  // println!("{}", fetched);
  // let s = match parse(&fetched).await {
  //   Ok(x) => x,
  //   Err(x) => return println!("Ошибка: {}", x),
  // };

  // // print_snapshot(&s)
  // println!("Date: {}", s.date)
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
