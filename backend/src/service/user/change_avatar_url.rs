use axum::Json;

use crate::{
    model::user::{UserChangeAvatarUrlPayload, UserClaim},
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn change_avatar_url(
    user_claim: UserClaim,
    Json(user_change_avatar_url_payload): Json<UserChangeAvatarUrlPayload>,
) -> AppResult<()> {
    let pool = database_connect();
    sql::user::update_avatar_url(
        pool,
        &user_claim.data.user_id,
        &user_change_avatar_url_payload.new,
    )
    .await?;
    Ok(AppResponse::success(None))
}
