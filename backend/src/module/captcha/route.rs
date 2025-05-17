use axum::{Router, routing::get};

use super::service;

pub fn init() -> Router {
    Router::new()
        .route("/email/{user_email}", get(service::email))
        .route("/image", get(service::image))
        .route("/phone/{user_phone}", get(service::phone))
}
