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
  parse(fetched);
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
