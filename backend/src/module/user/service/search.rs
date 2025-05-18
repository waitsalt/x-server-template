use axum::Json;

use crate::{
    common::response::AppResponse,
    module::{
        model::AppResult,
        user::{
            model::{UserPublic, UserSearchPayload},
            repository,
        },
    },
};

pub async fn search(
    Json(user_search_payload): Json<UserSearchPayload>,
) -> AppResult<Vec<UserPublic>> {
    let user_list =
        repository::select_user_where_user_name_like(&user_search_payload.keyword).await?;
    let mut user_public_list: Vec<UserPublic> = Vec::new();
    for user in user_list {
        user_public_list.push(UserPublic::from(user));
    }
    Ok(AppResponse::success(Some(user_public_list)))
}
