use super::*;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    // 获取路由
    let captcha_router = captcha::route::init();
    let user_router = user::route::init();

    // 设置请求允许
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 加载路由
    Router::new()
        .nest("/api/v0/captcha", captcha_router)
        .nest("/api/v0/user", user_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
