use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub async fn create_user(Json(body): Json<User>) -> Json<User> {
    Json(User { name: body.name})
}

