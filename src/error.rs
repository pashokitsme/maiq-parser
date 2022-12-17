#[derive(thiserror::Error, Debug)]
pub enum ParserError {
  #[error("Расписания ещё нет")]
  NotYet,

  #[error("Не удалось спарсить")]
  CantParse,

  #[error("{0}")]
  NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for ParserError {
  fn from(err: reqwest::Error) -> Self {
    ParserError::NetworkError(err)
  }
}
