use anyhow::Result;
use std::env;
use tracing::{debug, error, trace, warn};

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
                debug!("DATABASE_URL found");
                trace!("Value: {}", database_url);
                database_url
            }
            Err(e) => {
                error!("DATABASE_URL not found, exiting");
                return Err(e.into()); // Error here.
            }
        };

        // Retrieve PORT
        let port = if let Ok(port) = env::var("PORT") {
            debug!("PORT found");
            trace!("Value: {}", port);
            port
        } else {
            warn!("PORT not found, using default value 3000");
            "3000".to_string()
        };

        // Retrieve MANAGEMENT_SECRET
        let management_secret = match env::var("MANAGEMENT_SECRET") {
            Ok(management_secret) => {
                debug!("MANAGEMENT_SECRET found");
                trace!("Value: {}", management_secret);
                management_secret
            }
            Err(e) => {
                error!("MANAGEMENT_SECRET not set, exiting");
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
