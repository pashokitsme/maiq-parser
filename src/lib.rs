extern crate maiq_shared;

#[macro_use]
extern crate lazy_static;

pub use fetch::{fetch, Fetch, Fetched};
use log::info;
pub use maiq_shared::*;
pub use parser::parse;

use chrono::{DateTime, Utc};
use error::ParserError;

pub mod error;
pub mod fetch;
pub mod parser;
pub mod replacer;

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

pub fn warmup_defaults() {
  let group_names = replacer::REPLECEMENTS
    .iter()
    .map(|day| format!("{}: {}", day.day, day.groups.iter().map(|g| g.name.clone()).collect::<String>()))
    .collect::<Vec<String>>();

  info!("Loaded replacements for: {:?}", group_names);
}
