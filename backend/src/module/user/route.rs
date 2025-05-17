use axum::{
    Router,
    routing::{delete, get, post},
};

use super::service;

pub fn init() -> Router {
    Router::new()
        .route("/", post(service::create)) // 注册
        .route("/login", post(service::login)) // 登入
        .route("/logout", get(service::logout)) // 登出
        .route("/{user_id}", get(service::info)) // 用户信息
        .route("/{user_id}", delete(service::delete)) // 删除用户
        .route("/search", post(service::search)) // 搜索用户
        .route("/refresh", post(service::refresh_access_token)) // 刷新访问令牌
        .route("/change_password", post(service::change_password)) // 更改密码
        .route("/change_avatar_url", post(service::change_avatar_url)) // 更改用户头像
        .route("/change_email", post(service::change_email)) // 更改用户邮箱
        .route("/forget", post(service::forget)) // 忘记密码
}
