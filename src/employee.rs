use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
  pub Id: String,
  pub FullName: String,
  pub Location: String,
  pub JobTitle: String,
}
