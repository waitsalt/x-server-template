use axum::Json;
use redis::Commands;

use crate::{
    model::user::{UserChangeEmailPayload, UserClaim},
    sql,
    util::{
        AppResult, database::database_connect, error::AppError, redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn change_email(
    user_chaim: UserClaim,
    Json(user_change_email_payload): Json<UserChangeEmailPayload>,
) -> AppResult<()> {
    let mut con = redis_connect();
    let captcha_email_key = format!("captcha_email_key:{}", user_change_email_payload.user_email);
    let captcha_email: String = con
        .get_del(captcha_email_key)
        .map_err(|_| AppError::CaptchaEmailValueNotExist)?;
    if captcha_email != user_change_email_payload.captcha_email {
        return Err(AppError::CaptchaEmailValueError);
    }
    let pool = database_connect();
    sql::user::update_email(
        pool,
        &user_chaim.data.user_id,
        &user_change_email_payload.user_email,
    )
    .await?;
    Ok(AppResponse::success(None))
}
