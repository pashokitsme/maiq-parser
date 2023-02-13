#[cfg(not(feature = "__main"))]
fn main() {
  println!("Nothing here")
}

#[cfg(feature = "__main")]
/*
!          =================
!
! It's just a junk file for test something
! Too lazy to write tests
!
!          =================
 */
use maiq_parser::{fetch_snapshot, warmup_defaults, Fetch};
#[cfg(feature = "__main")]
use maiq_shared::Snapshot;

#[cfg(feature = "__main")]
#[allow(dead_code, unused_variables)]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  warmup_defaults();

  let snapshot = fetch_snapshot(Fetch::Today).await.unwrap();
  // println!("{:#?}", snapshot.group("Са1-21").unwrap());
  print_snapshot(&snapshot);
}

#[cfg(feature = "__main")]
#[allow(dead_code)]
fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.uid, s.date);
  for group in &s.groups {
    println!("Группа {} #{}", group.name, group.uid);
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
