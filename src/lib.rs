#[macro_use]
extern crate lazy_static;

use std::num::ParseIntError;

use log::info;
use maiq_shared::FetchUrl;

pub use maiq_shared::*;
use parser::table;
pub mod env;
pub mod parser;
pub mod replacer;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
  #[error("HTML Table not found")]
  NoTable,

  #[error("An unknown error occured: {0}")]
  Unknown(String),

  #[error("An reqwest error occured: {0}")]
  NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for ParserError {
  fn from(err: reqwest::Error) -> Self {
    ParserError::NetworkError(err)
  }
}

impl From<ParseIntError> for ParserError {
  fn from(err: ParseIntError) -> Self {
    ParserError::Unknown(err.to_string())
  }
}

pub async fn snapshot_from_remote<T: FetchUrl>(mode: &T) -> Result<Snapshot, ()> {
  let raw = fetch(mode).await.unwrap();
  let table = table::parse_html(&raw).unwrap();
  parser::snapshot::parse_snapshot(table)
}

pub fn warmup_defaults() {
  let group_names = replacer::REPLECEMENTS
    .iter()
    .map(|day| format!("{}: {}", day.day, day.groups.iter().map(|g| g.name.clone()).collect::<String>()))
    .collect::<Vec<String>>();

  info!("Loaded replacements for: {:?}", group_names);
}

pub async fn fetch<T: FetchUrl>(fetch_mode: &T) -> Result<String, reqwest::Error> {
  let res = reqwest::get(fetch_mode.url()).await?;
  let html = res.text_with_charset("windows-1251").await?;
  Ok(html)
}
