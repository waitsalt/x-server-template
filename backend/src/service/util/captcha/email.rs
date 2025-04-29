use axum::extract::Path;

use crate::{
    sql,
    util::{self, AppResult, database::database_connect, response::AppResponse},
};

pub async fn email(Path(user_email): Path<String>) -> AppResult<()> {
    let pool = database_connect();
    sql::user::user_email_is_exist(pool, &user_email).await?;
    util::email::captcha_email(&user_email).await?;
    Ok(AppResponse::success(None))
}
