mod user;
mod util;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    let user_router = user::init();
    let util_router = util::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .nest("/api/user", user_router)
        .nest("/api/util", util_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
