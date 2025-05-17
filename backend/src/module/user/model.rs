use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::util::config::CONFIG;

#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i64,
    pub user_name: String,
    pub user_desc: String,
    pub user_password: String,
    pub user_email: String,
    pub user_avatar_url: String, // 头像 url
    pub user_level: i16,         // 0
    pub user_status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub user_identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub user_create_time: DateTime<Utc>,
    pub user_update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPublic {
    pub user_id: i64,
    pub user_name: String,
    pub user_desc: String,
    pub user_email: String,
    pub user_avatar_url: String, // 头像 url
    pub user_level: i16,         // 0
    pub user_status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub user_identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub user_create_time: DateTime<Utc>,
    pub user_update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdatePayload {
    pub user_id: i64,
    pub user_name: String,
    pub user_desc: String,
    pub user_email: String,
    pub user_avatar_url: String, // 头像 url
    pub user_level: i16,         // 0
    pub user_status: i16,        // 0. 正常 1. 被封禁 2. 删除
    pub user_identity: i16,      // 0. 普通 1. 管理员 2. 超级管理员
    pub user_create_time: DateTime<Utc>,
    pub user_update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginPayload {
    pub user_name: String,
    pub user_password: String,
    pub captcha_image_key: String,
    pub captcha_image_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreatePayload {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub user_avatar_url: String,
    pub captcha_email: String,
    pub captcha_image_key: String,
    pub captcha_image_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChangePasswordPayload {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserChangeAvatarUrlPayload {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChangeEmailPayload {
    pub user_email: String,
    pub captcha_email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSearchPayload {
    pub keyword: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserForgetPayload {
    pub user_email: String,
    pub captcha_email: String,
    pub user_password: String,
    pub captcha_image_key: String,
    pub captcha_image_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserClaim {
    pub iat: i64,
    pub exp: i64,
    pub data: UserPublic,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRefreshClaim {
    pub iat: i64,
    pub exp: i64,
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserPublic {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        Self {
            user_id: user.user_id,
            user_name: user.user_name,
            user_desc: user.user_desc,
            user_email: user.user_email,
            user_avatar_url: user.user_avatar_url,
            user_level: user.user_level,
            user_status: user.user_status,
            user_identity: user.user_identity,
            user_create_time: user.user_create_time,
            user_update_time: user.user_update_time,
        }
    }
}

impl UserClaim {
    pub fn from(user: User) -> Self {
        let user = user.clone();
        let duration = CONFIG.auth.access_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::minutes(duration);
        Self {
            iat: start_time.timestamp_millis(),
            exp: end_time.timestamp_millis(),
            data: UserPublic::from(user),
        }
    }
}

impl UserRefreshClaim {
    pub fn new(data: &str) -> Self {
        let duration = CONFIG.auth.refresh_token_duration;
        let start_time = Utc::now();
        let end_time = start_time + Duration::days(duration);
        Self {
            iat: start_time.timestamp_millis(),
            exp: end_time.timestamp_millis(),
            data: data.to_string(),
        }
    }
}

impl UserAuth {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
