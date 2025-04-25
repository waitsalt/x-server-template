use axum::Json;

use crate::{
    model::user::{UserPublic, UserSearchPayload},
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn search(
    Json(user_search_payload): Json<UserSearchPayload>,
) -> AppResult<Vec<UserPublic>> {
    let pool = database_connect();
    let user_list = sql::user::user_search_by_name(pool, &user_search_payload.keyword).await?;
    let mut user_public_list: Vec<UserPublic> = Vec::new();
    for user in user_list {
        user_public_list.push(UserPublic::from(user));
    }
    Ok(AppResponse::success(Some(user_public_list)))
}
