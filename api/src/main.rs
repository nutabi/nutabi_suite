#![forbid(unsafe_code)]
#![warn(clippy::correctness, clippy::pedantic, clippy::style, clippy::perf)]

use anyhow::Context;
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Retrieve configuration
    let config = api::Config::new()?;

    // Initialise database connection
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("Unable to establish database connection")?;
    tracing::info!("Database connection established");

    // Create application
    let app = api::make_app().with_state(api::AppState { db_pool });

    // Create port listener
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .context("Cannot bind to port")?;

    // Start server
    axum::serve(listener, app).await.context("Server error")?;
    tracing::info!("Listening on {}", addr);

    Ok(())
}
