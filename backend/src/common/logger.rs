use std::env;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::config::CONFIG;

pub async fn init() {
    if env::var_os("RUST_LOG").is_none() {
        let app_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| CONFIG.server.name.clone());
        let level = CONFIG.logger.level.as_str();
        let env = format!("{app_name}={level},tower_http={level}");
        unsafe {
            env::set_var("RUST_LOG", env);
        }
    }

    tracing_subscriber::registry().with(fmt::layer()).init();
}
