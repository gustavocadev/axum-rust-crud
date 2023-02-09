use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BaseResponse<T> {
  pub status: String,
  pub data: Vec<T>,
}
