use maiq_shared::FetchUrl;

pub async fn fetch<T: FetchUrl>(fetch_mode: T) -> Result<String, reqwest::Error> {
  let res = reqwest::get(fetch_mode.url()).await?;
  let html = res.text_with_charset("windows-1251").await?;
  Ok(html)
}
