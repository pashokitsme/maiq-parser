#[macro_use]
extern crate lazy_static;

use chrono::Weekday;
use log::info;
use maiq_shared::FetchUrl;

pub use maiq_shared::*;
pub mod env;
pub mod parser;

pub async fn snapshot_from_remote<T: FetchUrl>(mode: &T) -> anyhow::Result<Snapshot> {
  let raw = fetch(mode).await?;
  let table = tl_table_parser::parse_last(&raw).ok_or_else(|| anyhow::anyhow!("Unable to parse table"))?;
  parser::snapshot::parse_snapshot(table, mode.date())
}

pub fn default_for(weekday: Weekday, group_name: &str) -> Option<&default::DefaultGroup> {
  parser::replace::REPLACEMENTS
    .iter()
    .find(|d| d.day == weekday)
    .and_then(|d| d.groups.iter().find(|g| g.name == group_name))
}

pub fn warmup_defaults() {
  let group_names = parser::replace::REPLACEMENTS
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
