mod v1;

use crate::AppState;
use axum::Router;

pub fn make_router() -> Router<AppState> {
    Router::new().nest("/v1", v1::make_router())
}
