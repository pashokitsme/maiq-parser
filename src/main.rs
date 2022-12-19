use maiq_parser::{
  fetch::{fetch, Fetch},
  parser::parse,
  timetable::{Day, LessonKind},
};

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

fn print_day(day: &Day) {
  println!("{} от {}\n", day.uid, day.date);
  for group in &day.groups {
    println!("Группа {}", group.name);
    for lesson in &group.lessons {
      print!("\t");
      match lesson.kind {
        LessonKind::None => println!("#{} Нет", lesson.num),
        LessonKind::Default => println!("#{} По расписанию", lesson.num),
        LessonKind::Some => {
          if let Some(sub) = lesson.subgroup {
            print!("Подгруппа {} ", sub)
          }
          println!(
            "#{} {} в {}. Преподаватель {}",
            lesson.num,
            lesson.name.as_ref().unwrap(),
            lesson.classroom.as_ref().unwrap_or(&"-".to_string()),
            lesson.teacher.as_ref().unwrap_or(&"-".to_string())
          )
        }
      }
    }

    println!()
  }
}

// fn main() {}
