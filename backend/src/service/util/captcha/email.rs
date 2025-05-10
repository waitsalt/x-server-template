use axum::extract::Path;

use crate::util::{self, AppResult, response::AppResponse};

pub async fn email(Path(user_email): Path<String>) -> AppResult<()> {
    util::email::captcha_email(&user_email).await?;
    Ok(AppResponse::success(None))
}
