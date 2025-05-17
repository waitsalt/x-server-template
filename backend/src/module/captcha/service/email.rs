use axum::extract::Path;

use crate::{
    module::model::AppResult,
    util::{self, response::AppResponse},
};

pub async fn email(Path(user_email): Path<String>) -> AppResult<()> {
    util::email::captcha_email(&user_email).await?;
    Ok(AppResponse::success(None))
}
