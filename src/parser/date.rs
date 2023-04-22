use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
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
