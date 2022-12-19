// use maiq_parser::{
//   fetch::{fetch, Fetch},
//   parser::parse,
//   timetable::Day,
// };

// #[tokio::main]
// async fn main() {
//   dotenvy::dotenv().unwrap();
//   let fetched = fetch(Fetch::Tomorrow).await.unwrap();
//   println!("{}", fetched);
//   let day = match parse(&fetched).await {
//     Ok(x) => x,
//     Err(x) => return println!("Ошибка: {}", x),
//   };

//   print_day(&day)
// }

// fn print_day(day: &Day) {
//   println!("{} от {}\n", day.uid, day.date);
//   for group in &day.groups {
//     println!("Группа {}", group.name);
//     for lesson in &group.lessons {
//       print!("\t#{} ", lesson.num);
//       if let Some(sub) = lesson.subgroup {
//         print!(" (п. {}) ", sub)
//       }
//       print!("{} ", lesson.name);

//       if let Some(classroom) = lesson.classroom.as_ref() {
//         print!(" в {}", classroom);
//       }

//       if let Some(teacher) = lesson.teacher.as_ref() {
//         print!(". Преподаватель: {}", teacher)
//       }
//       println!()
//     }
//     println!()
//   }
// }

fn main() {}
