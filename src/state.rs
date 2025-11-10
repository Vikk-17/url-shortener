use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub redis: deadpool_redis::Pool,
}
