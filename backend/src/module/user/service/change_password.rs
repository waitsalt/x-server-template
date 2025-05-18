use axum::Json;

use crate::{
    common::{error::AppError, response::AppResponse},
    module::{
        model::AppResult,
        user::{
            model::{UserChangePasswordPayload, UserClaim},
            repository,
        },
    },
};

pub async fn change_password(
    user_claim: UserClaim,
    Json(user_change_password_payload): Json<UserChangePasswordPayload>,
) -> AppResult<()> {
    let user = repository::select_user_where_user_id(user_claim.data.user_id).await?;
    if user.user_password != user_change_password_payload.old {
        return Err(AppError::UserPasswordError);
    }
    repository::update_user_set_user_password_where_user_id(
        user_claim.data.user_id,
        &user_change_password_payload.new,
    )
    .await?;
    Ok(AppResponse::success(None))
}
