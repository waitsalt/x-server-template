use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/signin", post(service::user::login)) // 登入
        .route("/signout", get(service::user::logout)) // 登出
        .route("/signup", post(service::user::create)) // 注册
        .route("/{user_id}", get(service::user::info)) // 用户信息
        .route("/{user_id}", delete(service::user::delete)) // 删除用户
        .route("/search", post(service::user::search)) // 搜索用户
        .route("/refresh", post(service::user::refresh_access_token)) // 刷新访问令牌
}
