use axum::Json;

use crate::{
    common::response::AppResponse,
    module::{
        model::AppResult,
        user::{
            model::{UserChangeAvatarUrlPayload, UserClaim},
            repository,
        },
    },
};

pub async fn change_avatar_url(
    user_claim: UserClaim,
    Json(user_change_avatar_url_payload): Json<UserChangeAvatarUrlPayload>,
) -> AppResult<()> {
    repository::update_user_set_user_avator_url_where_user_id(
        user_claim.data.user_id,
        &user_change_avatar_url_payload.new,
    )
    .await?;
    Ok(AppResponse::success(None))
}
