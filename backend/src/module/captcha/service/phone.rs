use crate::{common::response::AppResponse, module::model::AppResult};

pub async fn phone() -> AppResult<()> {
    Ok(AppResponse::success(None))
}
