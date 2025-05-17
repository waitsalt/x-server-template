use axum::extract::Path;

use crate::{
    module::{
        model::AppResult,
        user::{model::UserPublic, repository},
    },
    util::response::AppResponse,
};

pub async fn info(Path(user_id): Path<i64>) -> AppResult<UserPublic> {
    let user = repository::select_user_where_user_id(user_id).await?;
    let user_public = UserPublic::from(user);
    Ok(AppResponse::success(Some(user_public)))
}
