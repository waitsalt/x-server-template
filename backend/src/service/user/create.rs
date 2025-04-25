use axum::Json;

use crate::{
    model::user::UserSignupPayload,
    sql,
    util::{
        AppResult, database::database_connect, error::AppError, redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn create(Json(user_signup_payload): Json<UserSignupPayload>) -> AppResult<()> {
    // 检查密码
    if user_signup_payload.user_password.len() < 8 {
        return Err(AppError::UserPasswordShort);
    }

    let pool = database_connect();
    // 查询用户名是否存在
    sql::user::user_name_is_exist(pool, &user_signup_payload.user_name)
        .await
        .unwrap();

    // 查询用户邮箱是否存在
    sql::user::user_email_is_exist(pool, &user_signup_payload.user_email)
        .await
        .unwrap();

    // 验证邮箱验证码
    let mut con = redis_connect();

    let captcha_email_key = format!("captcha_email_key:{}", user_signup_payload.user_email);
    let captcha_email_value: String = match redis::cmd("GET")
        .arg(captcha_email_key.clone())
        .query(&mut con)
    {
        Ok(captcha_email) => captcha_email,
        Err(_) => "".to_string(),
    };

    if captcha_email_value != user_signup_payload.captcha_email {
        return Err(AppError::CaptchaEmailValueError);
    }

    // 邮箱验证码使用后失效
    let _: () = redis::cmd("DEL")
        .arg(captcha_email_key)
        .query(&mut con)
        .unwrap();

    // 新建用户
    sql::user::user_create(
        pool,
        &user_signup_payload.user_name,
        &user_signup_payload.user_password,
        &user_signup_payload.user_email,
        &user_signup_payload.user_avatar_url,
    )
    .await?;

    Ok(AppResponse::success(None))
}
