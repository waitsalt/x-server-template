use axum::Json;
use nanoid::nanoid;
use redis::Commands;

use crate::{
    common::{
        config::CONFIG, database::database_connect, error::AppError, redis::redis_connect,
        response::AppResponse,
    },
    module::{
        model::AppResult,
        user::{
            auth::{sign, sign_resfresh},
            model::{User, UserAuth, UserClaim, UserLoginPayload, UserRefreshClaim},
        },
    },
};

pub async fn login(Json(user_login_payload): Json<UserLoginPayload>) -> AppResult<UserAuth> {
    let mut con = redis_connect();

    // 验证图形验证码
    let captcha_image_key = format!("captcha_image_key:{}", user_login_payload.captcha_image_key);
    let captcha_image_value: String = con.get(&captcha_image_key).unwrap();
    let _: () = con.del(&captcha_image_key).unwrap();
    // .map_err(|_| AppError::RedisActionError)?;
    if captcha_image_value != user_login_payload.captcha_image_value {
        return Err(AppError::CaptchaImageValueError);
    }

    let pool = database_connect();

    // 验证用户密码
    let sql = "
        select
            *
        from
            \"user\"
        where
            user_name = $1
        ";
    let user: User = sqlx::query_as(sql)
        .bind(&user_login_payload.user_name)
        .fetch_one(pool)
        .await
        .unwrap();
    if user.user_status == 2 {
        return Err(AppError::UserIsDeleted);
    }
    if user.user_password != user_login_payload.user_password {
        return Err(AppError::UserPasswordError);
    }

    let random_data = nanoid!(16);
    let access_token = sign(UserClaim::from(user.clone()))?;
    let refresh_token = sign_resfresh(UserRefreshClaim::new(&random_data))?;

    let refresh_token_key = format!("refresh_token:{}", random_data);
    let refresh_token_duration = CONFIG.auth.refresh_token_duration;
    let _: () = con
        .set_ex(
            refresh_token_key,
            user.user_id,
            (refresh_token_duration * 3600 * 24) as u64,
        )
        .unwrap();

    Ok(AppResponse::success(Some(UserAuth::new(
        access_token,
        refresh_token,
    ))))
}
