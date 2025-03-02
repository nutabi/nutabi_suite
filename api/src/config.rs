use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        // Retrieve DATABASE_URL
        let database_url = match env::var("DATABASE_URL") {
            Ok(database_url) => {
                tracing::info!("DATABASE_URL found.");
                tracing::debug!("DATABASE_URL set to {}.", database_url);
                database_url
            },
            Err(e) => {
                tracing::error!("DATABASE_URL not set. Exiting.");
                return Err(e.into());
            }
        };

        // Retrieve PORT
        let port = match env::var("PORT") {
            Ok(port) => {
                tracing::info!("PORT set to {}.", port);
                port
            },
            Err(_) => {
                tracing::info!("PORT not set. Using default port 3000.");
                "3000".to_string()
            }
        };

        Ok(Self { database_url, port })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::MySqlPool,
}