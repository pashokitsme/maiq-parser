use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use table_extract::Row;

use super::into_text;

const MONTHS: [&str; 12] =
  ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];

pub fn parse_date(row: Row) -> Option<DateTime<Utc>> {
  let content = row.into_iter().map(|x| into_text(x)).collect::<String>();
  let mut split = content.split(' ');

  while let Some(word) = split.next() {
    let day = word.trim().parse::<u32>();
    if day.is_err() {
      continue;
    }

    let day = day.unwrap();

    let month = match split.next() {
      None => continue,
      Some(month) => MONTHS.iter().position(|&m| m == month).map(|x| x as u32 + 1),
    };

    let month = match month {
      Some(m) => m,
      None => continue,
    };

    let year = match split.next() {
      None => continue,
      Some(year) => {
        let y = &year.trim()[..4];
        match y.parse::<i32>() {
          Ok(y) => y,
          Err(_) => continue,
        }
      }
    };

    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    return Some(DateTime::from_utc(NaiveDateTime::new(date, NaiveTime::default()), Utc));
  }

  None
}
