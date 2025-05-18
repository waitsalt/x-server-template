use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};
use tokio::sync::OnceCell;

use super::config::CONFIG;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init() {
    let database_url = CONFIG.database.url.clone();

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("database init error");

    assert!(POOL.set(pool).is_ok(), "database init false");
}

pub fn database_connect() -> &'static Pool<Postgres> {
    POOL.get().expect("datebase pool get error")
}
