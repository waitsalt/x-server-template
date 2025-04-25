use crate::{
    model::user::UserRefreshClaim,
    util::{AppResult, redis::redis_connect, response::AppResponse},
};

pub async fn logout(user_refresh_claim: UserRefreshClaim) -> AppResult<()> {
    let mut con = redis_connect();

    println!("{:?}", user_refresh_claim);

    let user_id = user_refresh_claim.data;

    let token_key = format!("refresh_token:{}", user_id);

    let _: () = redis::cmd("DEL").arg(token_key).query(&mut con).unwrap();

    Ok(AppResponse::success(None))
}
