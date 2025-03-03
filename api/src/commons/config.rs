use anyhow::{Context, Result, anyhow};
use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine};
use hex::FromHex;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
    pub management_secret: [u8; 64],
    pub jwt_secret: [u8; 32],
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
        let management_secret: [u8; 64] = match env::var("MANAGEMENT_SECRET") {
            Ok(management_secret) => {
                tracing::info!("MANAGEMENT_SECRET found.");
                tracing::debug!("MANAGEMENT_SECRET set to {}.", management_secret);
                BASE64_URL_SAFE_NO_PAD
                    .decode(management_secret.as_bytes())
                    .context("Unable to decode MANAGEMENT_SECRET.")?
                    .try_into()
                    .map_err(|_| anyhow!("MANAGEMENT_SECRET is not 64 bytes long."))?
            }
            Err(e) => {
                tracing::error!("MANAGEMENT_SECRET not set. Exiting.");
                return Err(e.into()); // Error here.
            }
        };

        // Retrieve JWT_SECRET
        let jwt_secret: [u8; 32] = match env::var("JWT_SECRET") {
            Ok(jwt_secret) => {
                tracing::info!("JWT_SECRET found.");
                tracing::debug!("JWT_SECRET set to {}.", jwt_secret);
                Vec::from_hex(jwt_secret)
                    .context("Unable to decode JWT_SECRET.")?
                    .try_into()
                    .map_err(|_| anyhow!("JWT_SECRET is not 32 bytes long."))?
            }
            Err(e) => {
                tracing::error!("JWT_SECRET not set. Exiting.");
                return Err(e.into()); // Error here.
            }
        };

        Ok(Self {
            database_url,
            port,
            management_secret,
            jwt_secret,
        })
    }
}
