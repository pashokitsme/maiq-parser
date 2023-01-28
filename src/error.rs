use std::num::ParseIntError;

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
