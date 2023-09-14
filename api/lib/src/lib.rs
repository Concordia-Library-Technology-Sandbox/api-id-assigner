use sqlx::SqlitePool;

pub const API_VERSION: &str = "v0.0.1";

pub mod router;
mod node;


#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub domain: String,
}
