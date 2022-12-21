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

pub struct Parsed {
  pub raw: Fetched,
  pub snapshot: Snapshot,
  pub date: DateTime<Utc>,
}

pub async fn fetch_n_parse(mode: Fetch) -> Result<Parsed, ParserError> {
  let date = chrono::Utc::now();
  let raw = fetch(mode).await?;
  let snapshot = parse(&raw).await?;
  Ok(Parsed { raw, snapshot, date })
}