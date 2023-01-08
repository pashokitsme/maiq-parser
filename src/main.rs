use maiq_parser::warmup_defaults;
use maiq_shared::Snapshot;

// ? It's just a junk file for test something
// ? Too lazy to write tests

#[allow(dead_code, unused_variables)]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  warmup_defaults();
  // let a = replacer::REPLECEMENTS.clone();
  // let b = replacer::REPLECEMENTS.clone();
  // let c = replacer::REPLECEMENTS.clone();
  // println!("{:#?}", *replacer::REPLECEMENTS)
  // let date = Utc::now()
  //   .date_naive()
  //   .checked_add_days(Days::new(1))
  //   .unwrap()
  //   .and_hms_opt(0, 0, 0)
  //   .unwrap();
  // let utc = DateTime::<Utc>::from_utc(date, Utc);
  // let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
  // assert_eq!(date.timestamp(), utc.timestamp())
  // let fetched = fetch(Fetch::Today).await.unwrap();
  // let s = match parse(&fetched).await {
  //   Ok(x) => x,
  //   Err(x) => return println!("Ошибка: {}", x),
  // };

  // println!("parsed: {}, date: {}", s.parsed_date, s.date)
  // print_snapshot(&s)
}

#[allow(dead_code)]
fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.uid, s.date);
  for group in &s.groups {
    println!("Группа {}", group.name);
    for lesson in &group.lessons {
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
    println!()
  }
}
