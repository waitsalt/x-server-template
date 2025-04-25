pub mod auth;
pub mod config;
pub mod database;
pub mod email;
pub mod error;
pub mod logger;
pub mod phone;
pub mod redis;
pub mod response;

use error::AppError;
use response::AppResponse;

pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;

pub async fn init() {
    logger::init().await;
    tracing::info!("logger start success");
    database::init().await;
    tracing::info!("database start success");
    redis::init();
    tracing::info!("redis start success");

    tracing::info!("util mod start success");
}
