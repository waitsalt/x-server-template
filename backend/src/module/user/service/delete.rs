use axum::extract::Path;

use crate::{
    module::{
        model::AppResult,
        user::{model::UserClaim, repository},
    },
    util::{error::AppError, response::AppResponse},
};

pub async fn delete(user_claim: UserClaim, Path(user_id): Path<i64>) -> AppResult<()> {
    if user_claim.data.user_id != user_id {
        return Err(AppError::PermissionDenied);
    }
    repository::update_user_set_user_status_where_user_id(user_id, 2).await?;
    Ok(AppResponse::success(None))
}
