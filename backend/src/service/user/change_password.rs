use axum::Json;

use crate::{
    model::user::{UserChangePasswordPayload, UserClaim},
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

pub async fn change_password(
    user_claim: UserClaim,
    Json(user_change_password_payload): Json<UserChangePasswordPayload>,
) -> AppResult<()> {
    let pool = database_connect();
    let user = sql::user::user_info_get_by_id(pool, &user_claim.data.user_id).await?;
    if user.user_password != user_change_password_payload.old {
        return Err(AppError::UserPasswordError);
    }
    sql::user::update_password(
        pool,
        &user_claim.data.user_id,
        &user_change_password_payload.new,
    )
    .await?;
    Ok(AppResponse::success(None))
}
