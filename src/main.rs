// today https://rsp.chemk.org/4korp/today.htm
// tomorrow https://rsp.chemk.org/4korp/tomorrow.htm

use timetable::{Day, Group, Lesson};

mod timetable;
mod utils;

fn main() {
  test_day()
}

fn test_day() {
  let mut groups = vec![];
  for i in 2..5 {
    let mut lessons = vec![];
    for j in 0..3 {
      let lesson = Lesson { classroom: format!("Кабинет {}", 100 + j), name: format!("Пара #{}", j), num: j };
      lessons.push(lesson);
    }
    let group = Group { name: format!("Группа {}", i), lessons };
    groups.push(group)
  }
  let day = Day::new(groups, None);
  println!("{:#?}", day)
}
