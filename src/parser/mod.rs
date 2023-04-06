use chrono::{DateTime, Utc};
use maiq_shared::Snapshot;
use scraper::Html;

use crate::ParserError;

use self::date::parse_date;

mod date;
pub mod snapshot;
pub mod table;

pub fn parse(html: &str, possible_date: DateTime<Utc>) -> Result<Snapshot, ParserError> {
  todo!();
  // let table = table_extract::Table::find_first(html).ok_or(ParserError::NoTable)?;
  // let mut table = table.into_iter();
  // let mut lessons = vec![];
  // let mut prev = None;

  // let date = {
  //   let mut date = None;
  //   for _ in 0..2 {
  //     if let Some(d) = parse_date(table.next().unwrap()) {
  //       date = Some(d);
  //       break;
  //     }
  //   }

  //   let date = date.unwrap_or(possible_date);
  //   if possible_date > date {
  //     possible_date
  //   } else {
  //     date
  //   }
  // };

  // for row in table {
  //   let row = parse_row(row);
  //   let lesson = parse_lesson(row, &prev)?;
  //   if let Some(lesson) = lesson {
  //     prev = Some(lesson.clone());
  //     lessons.push(lesson);
  //   }
  // }

  // let groups = map_lessons_to_groups(lessons, date);

  // Ok(Snapshot::new(groups, date))
}

fn into_text(html: &str) -> String {
  let doc = Html::parse_document(html);
  let mut fragments = doc.root_element().text().peekable();
  let mut res = String::new();

  while let Some(fragment) = fragments.next() {
    let mut chars = fragment.chars().peekable();
    let mut whitespaces_only = true;
    while let Some(c) = chars.next() {
      let next = chars.peek();
      if whitespaces_only && !c.is_whitespace() {
        whitespaces_only = false;
      }

      if c.is_whitespace() && ((next.is_some() && next.unwrap().is_whitespace()) || next.is_none()) {
        continue;
      }

      match c {
        '\n' => (),
        c if c.is_whitespace() => res.push(' '),
        c => res.push(c),
      }
    }

    if whitespaces_only && fragments.peek().is_some() {
      res.push(' ')
    }
  }

  res
}
