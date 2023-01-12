use chrono::{DateTime, Datelike, Days, Duration, Timelike, Utc, Weekday};

static ALPHABET: &[u8] = "0123456789abcdefghijklmnopqrstuvwxyz".as_bytes();

#[macro_export]
macro_rules! num_as_bytes {
  ($n: expr, $tt:ty) => {{
    let mut res = [0u8; (<$tt>::BITS / 8) as usize];
    let mut n = $n;
    for i in 0..(<$tt>::BITS / 8) as usize {
      res[i] = ((n >> (8 * i)) & 0xff) as u8;
      n = n >> i;
    }
    res
  }};
}

pub(crate) fn bytes_as_str(bytes: &[u8]) -> String {
  let len = ALPHABET.len();
  let mut res = String::new();
  for b in bytes.chunks_exact(3) {
    let byte = b[0] as usize ^ b[1] as usize ^ b[2] as usize;
    let index = byte - (len * (byte / len));
    let ch = ALPHABET[index] as char;
    res.push(ch)
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

pub fn map_day<'d>(date: &DateTime<Utc>, day: &'d str) -> (DateTime<Utc>, i64) {
  let mut count: u64 = 0;
  let day = map_weekday(day);
  let mut days = date.date_naive().iter_days();
  for _ in 0..10 {
    if days.next().unwrap().weekday() == day {
      break;
    }
    count += 1;
  }
  (date.checked_add_days(Days::new(count)).unwrap(), count as i64)
}

pub fn now_date(offset_days: i64) -> DateTime<Utc> {
  now(offset_days)
    .with_hour(0)
    .unwrap()
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap()
    .with_nanosecond(0)
    .unwrap()
}

pub fn now(offset_days: i64) -> DateTime<Utc> {
  Utc::now() + Duration::hours(3) + Duration::days(offset_days)
}
