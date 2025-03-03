use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
}

impl Config {
    #[allow(clippy::missing_errors_doc)] // Error is well understood and intentional.
    pub fn new() -> anyhow::Result<Self> {
        // Retrieve DATABASE_URL
        let database_url = match env::var("DATABASE_URL") {
            Ok(database_url) => {
                tracing::info!("DATABASE_URL found.");
                tracing::debug!("DATABASE_URL set to {}.", database_url);
                database_url
            }
            Err(e) => {
                tracing::error!("DATABASE_URL not set. Exiting.");
                return Err(e.into()); // Error here.
            }
        };

        // Retrieve PORT
        let port = if let Ok(port) = env::var("PORT") {
            tracing::info!("PORT set to {}.", port);
            port
        } else {
            tracing::info!("PORT not set. Using default port 3000.");
            "3000".to_string()
        };

        Ok(Self { database_url, port })
    }
}
