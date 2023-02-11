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

pub mod time {
  use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

  pub fn now_with_offset(offset_days: i64) -> DateTime<Utc> {
    now() + Duration::days(offset_days)
  }

  pub fn now() -> DateTime<Utc> {
    Utc::now() + Duration::hours(3)
  }

  pub fn now_date() -> DateTime<Utc> {
    now()
      .with_hour(0)
      .unwrap()
      .with_minute(0)
      .unwrap()
      .with_second(0)
      .unwrap()
      .with_nanosecond(0)
      .unwrap()
  }

  pub fn now_date_offset(offset_days: i64) -> DateTime<Utc> {
    now_date() + Duration::days(offset_days)
  }

  pub fn is_week_even(date: &DateTime<Utc>) -> bool {
    date.iso_week().week0() % 2 == 0
  }
}
