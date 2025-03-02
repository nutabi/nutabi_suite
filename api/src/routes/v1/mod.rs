mod location;

use axum::Router;

use crate::config::AppState;

pub fn make_router() -> Router<AppState> {
    Router::new()
        .nest("/location", location::make_router())
}