use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route(
            "/captcha/email/{user_email}",
            get(service::util::captcha::email),
        )
        .route("/captcha/image", get(service::util::captcha::image))
        .route(
            "/captcha/phone/{user_phone}",
            get(service::util::captcha::phone),
        )
}
