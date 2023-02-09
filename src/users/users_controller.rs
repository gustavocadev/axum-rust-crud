use axum::{http::StatusCode, Json};
use uuid::Uuid;

// this are my own modules and structs
use super::dtos::CreateUserDto;
use super::entities::user::User;
use super::structs::BaseResponse;

// static mut USERS: Vec<User> = vec![
//   User {
//     id: Uuid::new_v4(),
//     name: "John Doe".to_string(),
//   },
//   User {
//     id: Uuid::new_v4(),
//     name: "Jane Doe".to_string(),
//   },
// ];

fn db() -> Vec<User> {
  let users: Vec<User> = vec![
    User {
      id: Uuid::new_v4(),
      name: "John Doe".to_string(),
    },
    User {
      id: Uuid::new_v4(),
      name: "Jane Doe".to_string(),
    },
  ];

  users
}

pub async fn get_users() -> (StatusCode, Json<BaseResponse<User>>) {
  (
    StatusCode::OK,
    Json(BaseResponse {
      status: "success".to_string(),
      data: db(),
    }),
  )
}

pub async fn create_user(Json(payload): Json<CreateUserDto>) -> (StatusCode, Json<CreateUserDto>) {
  println!("payload: {:#?}", payload);

  return (StatusCode::CREATED, Json(payload));
}
