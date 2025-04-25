use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route(
            "/captcha_email/{user_email}",
            get(service::util::captcha::email),
        )
        .route("/captcha_image", get(service::util::captcha::image))
        .route(
            "/captcha_phone/{user_phone}",
            get(service::util::captcha::phone),
        )
}
