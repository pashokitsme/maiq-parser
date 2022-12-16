#![allow(dead_code)]

use timetable::{Day, Group, Lesson};
use tokio;

use crate::parser::{fetch, parse, Fetch};

mod parser;
mod timetable;
mod utils;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();
  pretty_env_logger::init();
  let fetched = fetch(Fetch::Tomorrow).await.unwrap();
  println!("{}", fetched);
  let day = parse(fetched);
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
        println!(
          "\t#{} {} в {}. Преподаватель {}",
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

fn test_day() {
  let mut groups = vec![];
  for i in 2..5 {
    let mut lessons = vec![];
    for j in 0..3 {
      let lesson = Lesson {
        classroom: Some(format!("{}", 100 + j)),
        name: format!("Пара #{}", j),
        count: 1,
        num: j,
        teacher: Some(format!("Препод #{}", j)),
      };
      lessons.push(lesson);
    }
    let group = Group { name: format!("Группа {}", i), lessons };
    groups.push(group)
  }
  let day = Day::new(groups, None);
  println!("{:#?}", day)
}
