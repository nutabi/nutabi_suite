use crate::{AppState, commons::require_man::RequireMan, relations::DataSource};
use axum::{
    Json, Router,
    extract::{FromRequestParts, State},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
    response::IntoResponse,
    routing::{get, post},
};
use base64::{Engine, prelude::BASE64_URL_SAFE};
use rand::{RngCore, rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{debug, error, trace};

pub fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_data_source))
        .route("/", post(post_data_source))
}

async fn get_data_source(State(_): State<AppState>, data_source: DataSource) -> impl IntoResponse {
    debug!("Request received by get_data_source");
    trace!("Source: {}", data_source.name);

    let response = GetDataSourceResponse {
        name: data_source.name,
        description: data_source.description.unwrap_or_default(),
    };
    Json(response)
}

async fn post_data_source(
    State(state): State<AppState>,
    _: RequireMan,
    Json(payload): Json<PostDataSourcePayload>,
) -> impl IntoResponse {
    debug!("Request received by post_data_source");
    trace!("Source: {}", payload.name);

    // Generate a new access key
    let mut access_key: [u8; 32] = [0; 32];
    rng().fill_bytes(&mut access_key);

    // Calculate the hash of the access key
    let mut hasher = Sha256::new();
    hasher.update(access_key);
    let access_key_hash = BASE64_URL_SAFE.encode(hasher.finalize().as_slice());

    // Insert into the database
    match sqlx::query!(
        r#"
        INSERT INTO DataSource (Name, Description, AccessKey)
        VALUES (?, ?, ?);
        "#,
        payload.name,
        payload.description,
        access_key_hash
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            debug!("Data source inserted successfully.");
            trace!("Created: {}", payload.name);
            BASE64_URL_SAFE.encode(access_key).into_response()
        }
        Err(e) => {
            error!("Failed to insert data source");
            debug!("Error: {:?}", e);
            "Unable to create data source.".into_response()
        }
    }
}

#[derive(Deserialize)]
struct PostDataSourcePayload {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct GetDataSourceResponse {
    name: String,
    description: String,
}

// Implement the `FromRequestParts` trait for `DataSource`
// This will allow us to extract a `DataSource` from the request parts
impl FromRequestParts<AppState> for DataSource {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        debug!("Data source authentication request received");
        // Extract access token from the request header
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| {
                debug!("AUTHORIZATION header found");
                trace!("Value: {:?}", value);
                value.to_str().ok()
            })
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or_else(|| {
                debug!("Invalid AUTHORIZATION header");
                StatusCode::UNAUTHORIZED
            })?;

        debug!("Correctly formatted BEARER token found");
        trace!("Token: {}", token);

        // Decode the token
        let token = BASE64_URL_SAFE.decode(token.as_bytes()).map_err(|_| {
            debug!("Invalid base64 encoding");
            StatusCode::UNAUTHORIZED
        })?;

        // Calculate hash
        let mut hash = Sha256::new();
        hash.update(token);
        let hash = BASE64_URL_SAFE.encode(hash.finalize().as_slice());

        // Find the data source with the hash
        let data_source = sqlx::query_as!(
            DataSource,
            r#"
            SELECT
                SourceID AS source_id,
                CreatedAt AS created_at,
                LastUpdatedAt AS last_updated_at,
                Name AS name,
                Description AS description,
                AccessKey AS access_key
            FROM DataSource
            WHERE AccessKey = ?;
            "#,
            hash
        )
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| {
            debug!("Access key hash not found in database");
            trace!("Hash: {}", hash);
            StatusCode::UNAUTHORIZED
        })?;

        debug!("Data source found");
        trace!("Source: {}", data_source.name);
        Ok(data_source)
    }
}
