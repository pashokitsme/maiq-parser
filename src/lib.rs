#[macro_use]
extern crate lazy_static;

use log::info;
use maiq_shared::FetchUrl;

use error::ParserError;

pub use fetch::{fetch, Fetched};
pub use maiq_shared::*;
pub use parser::parse;

pub mod error;
pub mod fetch;
pub mod parser;
pub mod replacer;

pub async fn fetch_snapshot<T: FetchUrl>(mode: T) -> Result<Snapshot, ParserError> {
  let raw = fetch(mode).await?;
  parse(&raw)
}

pub fn warmup_defaults() {
  let group_names = replacer::REPLECEMENTS
    .iter()
    .map(|day| format!("{}: {}", day.day, day.groups.iter().map(|g| g.name.clone()).collect::<String>()))
    .collect::<Vec<String>>();

  info!("Loaded replacements for: {:?}", group_names);
}
