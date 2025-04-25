use crate::{
    model::user::{UserClaim, UserRefreshClaim},
    sql,
    util::{
        AppResult, auth::sign, database::database_connect, redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn refresh_access_token(user_refresh_claim: UserRefreshClaim) -> AppResult<String> {
    let mut con = redis_connect();
    let refresh_token_key = format!("refresh_token:{}", user_refresh_claim.data);
    let user_id: i64 = redis::cmd("GET")
        .arg(refresh_token_key)
        .query(&mut con)
        .unwrap();
    let pool = database_connect();
    let user = sql::user::user_info_get_by_id(pool, &user_id)
        .await
        .unwrap();
    let access_token = sign(UserClaim::from(user.clone()))?;

    Ok(AppResponse::success(Some(access_token)))
}
