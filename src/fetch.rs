use std::{fmt::Display, time::Duration};

use stopwatch::Stopwatch;

#[derive(Debug)]
pub enum Fetch {
  Today,
  Tomorrow,
}

impl Fetch {
  pub fn url(&self) -> &'static str {
    match self {
      Fetch::Tomorrow => "https://rsp.chemk.org/4korp/tomorrow.htm",
      Fetch::Today => "https://rsp.chemk.org/4korp/today.htm",
    }
  }
}

pub struct Fetched {
  pub html: String,
  pub took: Duration,
  pub etag: String,
  pub fetch_mode: Fetch,
}

impl Display for Fetched {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "Get {:?}. HTML: (...{}), etag: {}, took {}ms",
      self.fetch_mode,
      self.html.len(),
      self.etag,
      self.took.as_millis()
    ))
  }
}

pub async fn fetch(fetch_mode: Fetch) -> Result<Fetched, reqwest::Error> {
  let mut watch = Stopwatch::start_new();
  let res = reqwest::get(fetch_mode.url()).await?;
  let etag = res.headers().get("ETag").unwrap().to_str().unwrap().replace("\"", "");
  let html = res.text_with_charset("windows-1251").await?;
  watch.stop();
  Ok(Fetched { html, took: watch.elapsed(), etag, fetch_mode })
}
