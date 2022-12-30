extern crate maiq_structs;

pub use fetch::{fetch, Fetch, Fetched};
pub use maiq_structs::*;
pub use parser::parse;

use chrono::{DateTime, Utc};
use error::ParserError;

pub mod error;
pub mod fetch;
pub mod parser;

pub struct Parsed {
  pub raw: Fetched,
  pub snapshot: Snapshot,
  pub date: DateTime<Utc>,
}

pub async fn fetch_n_parse(mode: &Fetch) -> Result<Parsed, ParserError> {
  let date = chrono::Utc::now();
  let raw = fetch(mode.to_owned()).await?;
  let snapshot = parse(&raw).await?;
  Ok(Parsed { raw, snapshot, date })
}
