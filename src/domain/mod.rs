use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
  pub id: Option<i64>,
  pub title: String,
  pub contents: String,
}
