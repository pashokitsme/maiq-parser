use chrono::{DateTime, Datelike, Days, FixedOffset, Timelike, Utc, Weekday};

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

pub fn map_weekday<'d>(day: &'d str) -> Weekday {
  match day {
    "Понедельник" => Weekday::Mon,
    "Вторник" => Weekday::Tue,
    "Среда" => Weekday::Wed,
    "Четверг" => Weekday::Thu,
    "Пятница" => Weekday::Fri,
    "Суббота" => Weekday::Sat,
    "Воскресенье" => Weekday::Sun,
    _ => unimplemented!(),
  }
}

pub fn map_day<'d>(date: &DateTime<FixedOffset>, day: &'d str) -> DateTime<FixedOffset> {
  let mut count: u64 = 0;
  let day = map_weekday(day);
  let mut days = date.date_naive().iter_days();
  for _ in 0..10 {
    if days.next().unwrap().weekday() == day {
      break;
    }
    count += 1;
  }
  date.checked_add_days(Days::new(count as u64)).unwrap()
}

pub fn current_date(offset: u64) -> DateTime<FixedOffset> {
  Utc::now()
    .with_timezone(&FixedOffset::east_opt(3 * 60 * 60).unwrap())
    .with_hour(6)
    .unwrap()
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap()
    .with_nanosecond(0)
    .unwrap()
    .checked_add_days(Days::new(offset))
    .unwrap()
}
