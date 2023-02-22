#[cfg(not(feature = "__main"))]
fn main() {
  println!("Nothing here")
}

/*
!          =================
!
! It's just a junk file for test something
! Too lazy to write tests
!
!          =================
*/
#[cfg(feature = "__main")]
use maiq_parser::{fetch_snapshot, warmup_defaults, Fetch};

#[cfg(feature = "__main")]
use maiq_shared::{Snapshot, Group};

#[cfg(feature = "__main")]
#[allow(dead_code, unused_variables)]
#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  warmup_defaults();

  let snapshot = fetch_snapshot(Fetch::Next).await.unwrap();
  // println!("{:#?}", snapshot.group("Са1-21").unwrap());
  // print_snapshot(&snapshot);
  print_group(snapshot.group("Ир3-21").unwrap());
}

#[cfg(feature = "__main")]
#[allow(dead_code, unreachable_code)]
fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.uid, s.date);
  for group in &s.groups {
    print_group(group);
    println!()
  }
}

#[cfg(feature = "__main")]
fn print_group(g: &Group) {

  println!("Группа {} #{}", g.name, g.uid);
  for lesson in &g.lessons {
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
}