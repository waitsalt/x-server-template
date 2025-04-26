use axum::Json;

use crate::{
    model::user::{UserClaim, UserUpdatePayload},
    util::{AppResult, error::AppError, response::AppResponse},
};

pub async fn update(
    user_claim: UserClaim,
    Json(user_update_payload): Json<UserUpdatePayload>,
) -> AppResult<()> {
    if user_claim.data.user_id != user_update_payload.user_id {
        return Err(AppError::PermissionDenied);
    }
    Ok(AppResponse::success(None))
}
