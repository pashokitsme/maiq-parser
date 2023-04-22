#[macro_use]
extern crate lazy_static;

use log::info;
use maiq_shared::FetchUrl;

pub use maiq_shared::*;
use parser::table;
pub mod env;
pub mod parser;
pub mod replacer;

pub async fn snapshot_from_remote<T: FetchUrl>(mode: &T) -> anyhow::Result<Snapshot> {
  let raw = fetch(mode).await.unwrap();
  let table = table::parse_html(&raw).unwrap();
  parser::snapshot::parse_snapshot(table, mode.date())
}

pub fn warmup_defaults() {
  let group_names = replacer::REPLECEMENTS
    .iter()
    .map(|day| format!("{}: {}", day.day, day.groups.iter().map(|g| g.name.clone()).collect::<String>()))
    .collect::<Vec<String>>();

  info!("Loaded replacements for: {:?}", group_names);
}

pub async fn fetch<T: FetchUrl>(fetch_mode: &T) -> anyhow::Result<String> {
  let res = reqwest::get(fetch_mode.url()).await?;
  let html = res.text_with_charset("windows-1251").await?;
  Ok(html)
}
