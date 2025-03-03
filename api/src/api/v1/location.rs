use crate::{AppState, relations::DataSource};
use axum::{Json, Router, extract::State, response::IntoResponse, routing};
use serde::Deserialize;
use tracing::{debug, error, trace};

pub fn make_router() -> Router<AppState> {
    Router::new().route("/", routing::post(post_location))
}

#[derive(Deserialize)]
pub struct PostLocationPayload {
    pub latitude: f64,
    pub longitude: f64,
}

async fn post_location(
    State(state): State<AppState>,
    data_source: DataSource,
    Json(payload): Json<PostLocationPayload>,
) -> impl IntoResponse {
    debug!("Request received by post_location");
    trace!("Source: {}", data_source.name);

    // Execute the query
    match sqlx::query("INSERT INTO Location (Latitude, Longitude) VALUES (?, ?)")
        .bind(payload.latitude)
        .bind(payload.longitude)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => {
            debug!("Location inserted successfully.");
            "Location inserted successfully."
        }
        Err(e) => {
            error!("Failed to insert location");
            debug!("Error: {:?}", e);
            "Failed to insert location."
        }
    }
}
