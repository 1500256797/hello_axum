use axum::extract::FromRef;
use bb8::Pool;
use sqlx::PgPool;

use crate::redis_manager::RedisConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: Pool<RedisConnectionManager>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for Pool<RedisConnectionManager> {
    fn from_ref(app_state: &AppState) -> Pool<RedisConnectionManager> {
        app_state.redis_pool.clone()
    }
}
