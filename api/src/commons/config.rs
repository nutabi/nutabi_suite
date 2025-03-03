use anyhow::Result;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
    pub management_secret: String,
}

impl Config {
    #[allow(clippy::missing_errors_doc)] // Error is well understood and intentional.
    pub fn new() -> Result<Self> {
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

        // Retrieve MANAGEMENT_SECRET
        let management_secret = match env::var("MANAGEMENT_SECRET") {
            Ok(management_secret) => {
                tracing::info!("MANAGEMENT_SECRET found.");
                tracing::debug!("MANAGEMENT_SECRET set to {}.", management_secret);
                management_secret
            }
            Err(e) => {
                tracing::error!("MANAGEMENT_SECRET not set. Exiting.");
                return Err(e.into()); // Error here.
            }
        };

        Ok(Self {
            database_url,
            port,
            management_secret,
        })
    }
}
