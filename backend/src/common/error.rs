use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // 系统错误
    ConfigError,
    SqlActionError,
    RedisActionError,

    // 认证与鉴权错误
    InvalidToken,
    PermissionDenied,

    // 验证码相关错误
    ImageSendFail,
    EmailSendFail,
    PhoneSendFail,
    CaptchaImageValueError,
    CaptchaEmailValueError,
    CaptchaPhoneValueError,
    CaptchaImageValueNotExist,
    CaptchaEmailValueNotExist,
    CaptchaPhoneValueNotExist,

    // 用户相关错误
    UserExist,
    UserNameExist,
    UserEmailExist,
    UserPhoneExist,
    UserNotExist,
    UserNameNotExist,
    UserEmailNotExist,
    UserPhoneNotExist,
    UserPasswordError,
    UserPasswordShort,
    UserIsDeleted,

    // 图书
    BookExist,
    BookNotExist,

    // 收藏
    CollectExist,
    CollectNotExist,
    CollectBookExist,
    CollectBookNotExist,

    // 历史
    HistoryExist,
    HistoryNotExist,
    HistoryBookExist,
    HistoryBookNotExist,

    // 其他问题
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

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::SqlActionError
    }
}

impl From<redis::RedisError> for AppError {
    fn from(_: redis::RedisError) -> Self {
        AppError::RedisActionError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            // 系统错误
            AppError::ConfigError => (StatusCode::INTERNAL_SERVER_ERROR, 1001, "服务配置文件错误"),
            AppError::SqlActionError => (StatusCode::INTERNAL_SERVER_ERROR, 1002, "数据库操作错误"),
            AppError::RedisActionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, 1003, "Redis 操作错误")
            }

            // 认证与鉴权错误
            AppError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, 2001, "Token 无效、过期或签名错误")
            }
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, 2002, "权限不足"),

            // 验证码相关错误
            AppError::ImageSendFail => (StatusCode::INTERNAL_SERVER_ERROR, 3001, "图片发送失败"),
            AppError::EmailSendFail => (StatusCode::INTERNAL_SERVER_ERROR, 3002, "邮件发送失败"),
            AppError::PhoneSendFail => (StatusCode::INTERNAL_SERVER_ERROR, 3003, "短信发送失败"),
            AppError::CaptchaImageValueError => (StatusCode::BAD_REQUEST, 3011, "图形验证码错误"),
            AppError::CaptchaEmailValueError => (StatusCode::BAD_REQUEST, 3012, "邮箱验证码错误"),
            AppError::CaptchaPhoneValueError => (StatusCode::BAD_REQUEST, 3013, "短信验证码错误"),
            AppError::CaptchaImageValueNotExist => {
                (StatusCode::BAD_REQUEST, 3021, "图形验证码不存在")
            }
            AppError::CaptchaEmailValueNotExist => {
                (StatusCode::BAD_REQUEST, 3022, "邮箱验证码不存在")
            }
            AppError::CaptchaPhoneValueNotExist => {
                (StatusCode::BAD_REQUEST, 3023, "短信验证码不存在")
            }

            // 用户相关错误
            AppError::UserExist => (StatusCode::BAD_REQUEST, 4001, "用户已存在"),
            AppError::UserNameExist => (StatusCode::BAD_REQUEST, 4002, "用户名已被注册"),
            AppError::UserEmailExist => (StatusCode::BAD_REQUEST, 4003, "邮箱已被注册"),
            AppError::UserPhoneExist => (StatusCode::BAD_REQUEST, 4004, "手机号已被注册"),
            AppError::UserNotExist => (StatusCode::BAD_REQUEST, 4005, "用户不存在"),
            AppError::UserNameNotExist => (StatusCode::BAD_REQUEST, 4006, "用户名不存在"),
            AppError::UserEmailNotExist => (StatusCode::BAD_REQUEST, 4007, "邮箱不存在"),
            AppError::UserPhoneNotExist => (StatusCode::BAD_REQUEST, 4008, "手机号不存在"),
            AppError::UserPasswordError => (StatusCode::BAD_REQUEST, 4009, "密码错误"),
            AppError::UserPasswordShort => (StatusCode::BAD_REQUEST, 4010, "密码太短"),
            AppError::UserIsDeleted => (StatusCode::BAD_REQUEST, 4011, "用户已被删除"),

            // 书相关错误
            AppError::BookExist => (StatusCode::BAD_REQUEST, 5001, "书已存在"),
            AppError::BookNotExist => (StatusCode::BAD_REQUEST, 5002, "书不存在"),

            // 收藏相关问题
            AppError::CollectExist => (StatusCode::BAD_REQUEST, 6001, "收藏列表已存在"),
            AppError::CollectNotExist => (StatusCode::BAD_REQUEST, 6002, "收藏列表不存在"),
            AppError::CollectBookExist => (StatusCode::BAD_REQUEST, 6003, "收藏列表中已有该书"),
            AppError::CollectBookNotExist => (StatusCode::BAD_REQUEST, 6004, "收藏列表中没有该书"),

            // 历史相关问题
            AppError::HistoryExist => (StatusCode::BAD_REQUEST, 7001, "历史记录已存在"),
            AppError::HistoryNotExist => (StatusCode::BAD_REQUEST, 7002, "历史记录不存在"),
            AppError::HistoryBookExist => (StatusCode::BAD_REQUEST, 7003, "历史记录中已有该书"),
            AppError::HistoryBookNotExist => (StatusCode::BAD_REQUEST, 7004, "历史记录中没有该书"),

            // 其他问题
            // AppError::Other => (StatusCode::FORBIDDEN, 9000, "未知错误"),
            _ => (StatusCode::FORBIDDEN, 9000, "未知错误"),
        };
        let body = Json(json!({
            "code": code,
            "message":message,
            "data":()
        }));
        (status_code, body).into_response()
    }
}
