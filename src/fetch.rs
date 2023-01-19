use maiq_shared::FetchUrl;

pub struct Fetched {
  pub html: String,
  pub etag: String,
}

pub async fn fetch<T: FetchUrl>(fetch_mode: T) -> Result<Fetched, reqwest::Error> {
  let res = reqwest::get(fetch_mode.url()).await?;
  let etag = res.headers().get("ETag").unwrap().to_str().unwrap().replace("\"", "");
  let html = res.text_with_charset("windows-1251").await?;
  Ok(Fetched { html, etag })
}
