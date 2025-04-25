use axum::extract::Path;

use crate::{
    model::user::UserPublic,
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn info(Path(user_id): Path<i64>) -> AppResult<UserPublic> {
    let pool = database_connect();
    let user = sql::user::user_info_get_by_id(pool, &user_id).await?;
    let user_public = UserPublic::from(user);
    Ok(AppResponse::success(Some(user_public)))
}
