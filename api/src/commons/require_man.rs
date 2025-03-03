use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};

use crate::AppState;

pub struct RequireMan;

impl FromRequestParts<AppState> for RequireMan {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        {
            Some(token) if token == state.management_secret => Ok(RequireMan),
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
