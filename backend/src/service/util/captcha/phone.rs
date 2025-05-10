use crate::util::{AppResult, response::AppResponse};

pub async fn phone() -> AppResult<()> {
    Ok(AppResponse::success(None))
}
