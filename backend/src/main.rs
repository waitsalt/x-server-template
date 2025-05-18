mod common;
mod module;

use common::config::CONFIG;

#[tokio::main]
async fn main() {
    // 初始化项目功能模块
    common::init().await;

    let address = format!("{}:{}", CONFIG.server.address, CONFIG.server.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("address bind error");
    let router = module::route::init();
    tracing::info!("Server start: http://{}", address);
    axum::serve(listener, router).await.expect("app run error")
}
