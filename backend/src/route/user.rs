use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", post(service::user::create)) // 注册
        .route("/login", post(service::user::login)) // 登入
        .route("/logout", get(service::user::logout)) // 登出
        .route("/{user_id}", get(service::user::info)) // 用户信息
        .route("/{user_id}", delete(service::user::delete)) // 删除用户
        .route("/search", post(service::user::search)) // 搜索用户
        .route("/refresh", post(service::user::refresh_access_token)) // 刷新访问令牌
        .route("/change_password", post(service::user::change_password)) // 更改密码
        .route("/change_avatar_url", post(service::user::change_avatar_url)) // 更改用户头像
        .route("/change_email", post(service::user::change_email)) // 更改用户邮箱
}
