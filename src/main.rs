use std::net::SocketAddr;

use axum::routing::{get, post, Router};

mod users;
use users::users_controller::{create_user, get_users};

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/api/users", get(get_users))
    .route("/api/users", post(create_user));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
