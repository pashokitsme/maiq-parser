use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
  #[error("Расписания ещё нет")]
  NotYet,

  #[error("Не удалось спарсить: {0}")]
  CantParse(String),

  #[error("{0}")]
  NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for ParserError {
  fn from(err: reqwest::Error) -> Self {
    ParserError::NetworkError(err)
  }
}

impl From<ParseIntError> for ParserError {
  fn from(err: ParseIntError) -> Self {
    ParserError::CantParse(err.to_string())
  }
}
