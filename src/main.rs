use maiq_parser::{
  fetch::{fetch, Fetch},
  parser::parse,
  timetable::Day,
};

#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  pretty_env_logger::init();
  let fetched = fetch(Fetch::Today).await.unwrap();
  println!("{}", fetched);
  let day = match parse(&fetched).await {
    Ok(x) => x,
    Err(x) => return println!("Ошибка: {}", x),
  };

  print_day(&day)
}

fn print_day(day: &Day) {
  println!("{} от {}\n", day.hash, day.date);
  for group in &day.groups {
    println!("Группа {}", group.name);
    for lesson in &group.lessons {
      for i in 0..lesson.count {
        if lesson.classroom.is_none() {
          println!("\t#{} {}", lesson.num + 1, lesson.name);
          continue;
        }
        print!("\t");
        if let Some(sub) = lesson.subgroup {
          print!("Подгруппа {} ", sub)
        }
        println!(
          "#{} {} в {}. Преподаватель {}",
          lesson.num + i,
          lesson.name,
          lesson.classroom.as_ref().unwrap_or(&"-".to_string()),
          lesson.teacher.as_ref().unwrap_or(&"-".to_string())
        )
      }
    }

    println!()
  }
}
