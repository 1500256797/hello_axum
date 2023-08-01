use crate::redis_manager::RedisConnectionManager;
use axum::extract::FromRef;
use axum_jwt_auth::JwtDecoderState;
use bb8::Pool;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub jwt_decoder: JwtDecoderState,
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

impl FromRef<AppState> for JwtDecoderState {
    fn from_ref(app_state: &AppState) -> JwtDecoderState {
        app_state.jwt_decoder.clone()
    }
}
