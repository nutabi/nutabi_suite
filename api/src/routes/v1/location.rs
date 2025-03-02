use axum::{extract::State, response::IntoResponse, routing, Json, Router};
use serde::Deserialize;

use crate::config::AppState;

pub fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(post_location))
}

#[derive(Deserialize)]
pub struct PostLocationPayload {
    pub latitude: f64,
    pub longitude: f64,
}

async fn post_location(
    State(state): State<AppState>,
    Json(payload): Json<PostLocationPayload>,
) -> impl IntoResponse {
    // Execute the query
    match sqlx::query("INSERT INTO Location (Latitude, Longitude) VALUES (?, ?)")
        .bind(payload.latitude)
        .bind(payload.longitude)
        .execute(&state.db_pool)
        .await {
            Ok(_) => {
                tracing::info!("Location inserted successfully.");
                "Location inserted successfully."
            },
            Err(e) => {
                tracing::error!("Failed to insert location: {:?}", e);
                "Failed to insert location."
            }
        }
}