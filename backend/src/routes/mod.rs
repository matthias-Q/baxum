mod user;

use axum::{
    body::Body,
    routing::post,
    Router,
};

use self::user::create_user;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/user", post(create_user))
}
