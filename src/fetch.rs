use std::time::Duration;

use maiq_shared::FetchUrl;
use stopwatch::Stopwatch;

pub struct Fetched {
  pub html: String,
  pub took: Duration,
  pub etag: String,
}

pub async fn fetch<T: FetchUrl>(fetch_mode: T) -> Result<Fetched, reqwest::Error> {
  let mut watch = Stopwatch::start_new();
  let res = reqwest::get(fetch_mode.url()).await?;
  let etag = res.headers().get("ETag").unwrap().to_str().unwrap().replace("\"", "");
  let html = res.text_with_charset("windows-1251").await?;
  watch.stop();
  Ok(Fetched { html, took: watch.elapsed(), etag })
}
