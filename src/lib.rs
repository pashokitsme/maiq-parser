use chrono::{DateTime, Utc};
use error::ParserError;
use fetch::{fetch, Fetch, Fetched};
use parser::parse;
use timetable::Day;

pub mod error;
pub mod fetch;
pub mod parser;
pub mod timetable;
mod utils;

pub struct ParsedDay {
  pub fetched: Fetched,
  pub day: Day,
  pub date: DateTime<Utc>,
}

pub async fn fetch_n_parse(mode: Fetch) -> Result<ParsedDay, ParserError> {
  let date = chrono::Utc::now();
  let fetched = fetch(mode).await?;
  let day = parse(&fetched).await?;
  Ok(ParsedDay { fetched, day, date })
}
