use crate::{
    module::{
        model::AppResult,
        user::{
            auth::sign,
            model::{UserClaim, UserRefreshClaim},
            repository,
        },
    },
    util::{redis::redis_connect, response::AppResponse},
};

pub async fn refresh_access_token(user_refresh_claim: UserRefreshClaim) -> AppResult<String> {
    let mut con = redis_connect();
    let refresh_token_key = format!("refresh_token:{}", user_refresh_claim.data);
    let user_id: i64 = redis::cmd("GET").arg(refresh_token_key).query(&mut con)?;
    let user = repository::select_user_where_user_id(user_id).await?;
    let access_token = sign(UserClaim::from(user.clone()))?;

    Ok(AppResponse::success(Some(access_token)))
}
