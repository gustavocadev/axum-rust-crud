use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDto {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsersDto {
  pub id: u64,
  pub name: String,
}
