pub mod config;
pub mod database;
pub mod error;
pub mod logger;
pub mod redis;
pub mod response;

pub async fn init() {
    logger::init().await;
    tracing::info!("logger start success");
    database::init().await;
    tracing::info!("database start success");
    redis::init();
    tracing::info!("redis start success");

    tracing::info!("util mod start success");
}
