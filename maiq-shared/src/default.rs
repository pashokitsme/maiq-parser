use chrono::Weekday;
use serde::{Deserialize, Serialize};

use crate::Num;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultDay {
  pub day: Weekday,
  pub groups: Vec<DefaultGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultGroup {
  pub name: String,
  pub lessons: Vec<DefaultLesson>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultLesson {
  #[serde(skip_serializing_if = "Num::is_none")]
  pub num: Num,
  
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
