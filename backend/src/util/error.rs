use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // 服务配置
    ConfigError,

    // 鉴权
    InvalidToken,
    CaptchaImageValueError,

    // sql
    UserNotExist,
    UserPasswordError,
    UserNameExist,
    UserEmailExist,
    UserPhoneExist,
    UserPasswordShort,
    CaptchaEmailValueError,

    // other
    EmailSendFail,
    Other,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(_: config::ConfigError) -> Self {
        AppError::ConfigError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            AppError::ConfigError => (StatusCode::RANGE_NOT_SATISFIABLE, 10001, "服务配置文件错误"),
            AppError::InvalidToken => (StatusCode::INTERNAL_SERVER_ERROR, 2001, "token 错误"),
            AppError::CaptchaImageValueError => {
                (StatusCode::BAD_REQUEST, 2002, "图形验证码输入错误")
            }
            AppError::UserNotExist => (StatusCode::FORBIDDEN, 3001, "没有该用户"),
            AppError::UserPasswordError => (StatusCode::BAD_REQUEST, 3002, "用户密码错误"),
            AppError::UserNameExist => (StatusCode::BAD_REQUEST, 3003, "用户名已存在"),
            AppError::UserEmailExist => (StatusCode::BAD_REQUEST, 3003, "邮箱已注册"),
            AppError::UserPhoneExist => (StatusCode::BAD_REQUEST, 3003, "手机号已注册"),
            AppError::UserPasswordShort => (StatusCode::BAD_REQUEST, 3004, "密码至少八位"),
            AppError::CaptchaEmailValueError => (StatusCode::BAD_REQUEST, 3004, "验邮箱证码错误"),
            AppError::EmailSendFail => (StatusCode::SERVICE_UNAVAILABLE, 4001, "邮件发送失败"),

            AppError::Other => (StatusCode::FORBIDDEN, 5000, "未知错误"),
        };
        let body = Json(json!({
            "code": code,
            "message":message,
            "data":()
        }));
        (status_code, body).into_response()
    }
}
