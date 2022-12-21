use chrono::{Datelike, Days, NaiveDate, Weekday};

const SIZE_OF_USIZE: usize = (usize::BITS / 8) as usize;

pub fn usize_as_bytes(n: usize) -> [u8; SIZE_OF_USIZE] {
  let mut res = [0u8; SIZE_OF_USIZE];
  let mut n = n;
  for i in 0..SIZE_OF_USIZE {
    res[i] = ((n >> (8 * i)) & 0xff) as u8;
    n = n >> i;
  }
  res
}

pub fn map_weekday<'d>(day: Weekday) -> &'d str {
  match day {
    Weekday::Mon => "Понедельник",
    Weekday::Tue => "Вторник",
    Weekday::Wed => "Среда",
    Weekday::Thu => "Четверг",
    Weekday::Fri => "Пятница",
    Weekday::Sat => "Суббота",
    Weekday::Sun => "Воскресенье",
  }
}

pub fn map_day<'d>(date: &NaiveDate, day: &'d str) -> NaiveDate {
  let mut max = 10;
  let count = date
    .iter_days()
    .take_while(|x| {
      max -= 1;
      map_weekday(x.weekday()) == day || max == 0
    })
    .count();
  date.checked_add_days(Days::new(count as u64)).unwrap()
}
