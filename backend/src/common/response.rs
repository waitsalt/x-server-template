use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> AppResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self {
            code: 200,
            message: "success".into(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
