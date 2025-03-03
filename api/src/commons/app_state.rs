use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}
