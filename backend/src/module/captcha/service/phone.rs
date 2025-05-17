use crate::{module::model::AppResult, util::response::AppResponse};

pub async fn phone() -> AppResult<()> {
    Ok(AppResponse::success(None))
}
