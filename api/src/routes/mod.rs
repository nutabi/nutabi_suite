mod v1;

use axum::Router;

use crate::config::AppState;

pub fn make_router() -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::make_router())
}