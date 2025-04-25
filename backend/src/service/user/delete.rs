use axum::extract::Path;

use crate::{
    model::user::UserClaim,
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

pub async fn delete(user_claim: UserClaim, Path(user_id): Path<i64>) -> AppResult<()> {
    if user_claim.data.user_id != user_id {
        return Err(AppError::InvalidToken);
    }
    let pool = database_connect();
    let _ = sql::user::user_delete(pool, &user_id).await?;
    return Ok(AppResponse::success(None));
}
