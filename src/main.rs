#[cfg(feature = "main_fn")]
use maiq_parser::{
  fetch::{fetch, Fetch},
  parser::parse,
  timetable::Day,
};

#[cfg(feature = "main_fn")]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  let fetched = fetch(Fetch::Tomorrow).await.unwrap();
  println!("{}", fetched);
  let day = match parse(&fetched).await {
    Ok(x) => x,
    Err(x) => return println!("Ошибка: {}", x),
  };

  print_day(&day)
}

#[cfg(feature = "main_fn")]
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

#[cfg(not(feature = "main_fn"))]
fn main() {}
