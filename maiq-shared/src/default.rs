use chrono::Weekday;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultDay {
  pub day: Weekday,
  pub groups: Vec<DefaultGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DefaultGroup {
  pub name: String,
  pub lessons: Vec<DefaultLesson>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DefaultLesson {
  pub num: u8,
  pub name: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_even: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub subgroup: Option<u8>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub teacher: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub classroom: Option<String>,
}
