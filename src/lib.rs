pub use fetch::{fetch, Fetch, Fetched};
pub use parser::parse;

use chrono::{DateTime, Utc};
use error::ParserError;
use timetable::Snapshot;

pub mod error;
pub mod fetch;
pub mod parser;
pub mod timetable;

mod utils;

pub struct ParsedDay {
  pub fetched: Fetched,
  pub day: Snapshot,
  pub date: DateTime<Utc>,
}

pub async fn fetch_n_parse(mode: Fetch) -> Result<ParsedDay, ParserError> {
  let date = chrono::Utc::now();
  let fetched = fetch(mode).await?;
  let day = parse(&fetched).await?;
  Ok(ParsedDay { fetched, day, date })
}
