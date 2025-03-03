use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use tracing::{debug, trace};

use crate::AppState;

pub struct RequireMan;

impl FromRequestParts<AppState> for RequireMan {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        debug!("Management authentication request received");
        match parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| {
                debug!("AUTHORIZATION header found");
                trace!("Value: {:?}", v);
                v.to_str().ok()
            })
            .and_then(|value| value.strip_prefix("Bearer "))
        {
            Some(token) if token == state.management_secret => {
                debug!("Valid management secret validated");
                Ok(RequireMan)
            }
            _ => {
                debug!("Invalid management secret");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}
