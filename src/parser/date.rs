use chrono::{DateTime, Datelike, Utc};
use maiq_shared::utils::time::now_date;

const MONTHS: [&str; 12] =
  ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];

pub fn parse_date<T: Iterator<Item = Vec<String>>>(row: &mut T) -> Option<DateTime<Utc>> {
  let x = row.next().unwrap();
  let mut split = x.first().unwrap().split(' ');

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

    return now_date().with_day(day).unwrap().with_month(month);
  }

  None
}
