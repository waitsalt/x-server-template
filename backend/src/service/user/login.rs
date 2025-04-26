use axum::Json;
use nanoid::nanoid;
use redis::Commands;

use crate::{
    model::user::{UserAuth, UserClaim, UserCreatePayload, UserRefreshClaim},
    sql,
    util::{
        AppResult,
        auth::{sign, sign_resfresh},
        config::CONFIG,
        database::database_connect,
        error::AppError,
        redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn login(Json(user_login_payload): Json<UserCreatePayload>) -> AppResult<UserAuth> {
    let mut con = redis_connect();

    // 验证图形验证码
    let captcha_image_key = format!("captcha_image_key:{}", user_login_payload.captcha_image_key);
    let captcha_image_value: String = con
        .get_del(captcha_image_key)
        .map_err(|_| AppError::RedisActionError)?;
    if captcha_image_value != user_login_payload.captcha_image_value {
        return Err(AppError::CaptchaImageValueError);
    }

    let pool = database_connect();

    // 验证用户密码
    let user = sql::user::user_info_get_by_name(pool, &user_login_payload.user_name).await?;
    if user.user_password != user_login_payload.user_password {
        return Err(AppError::UserPasswordError);
    }

    let random_data = nanoid!(16);
    let access_token = sign(UserClaim::from(user.clone()))?;
    let refresh_token = sign_resfresh(UserRefreshClaim::new(&random_data))?;

    let refresh_token_key = format!("refresh_token:{}", random_data);
    let refresh_token_duration = CONFIG.auth.refresh_token_duration;
    let _: () = redis::cmd("SET")
        .arg(refresh_token_key)
        .arg(user.user_id)
        .arg("EX")
        .arg(refresh_token_duration * 3600 * 24)
        .query(&mut con)?;
    // .unwrap();?;

    return Ok(AppResponse::success(Some(UserAuth::new(
        access_token,
        refresh_token,
    ))));
}
