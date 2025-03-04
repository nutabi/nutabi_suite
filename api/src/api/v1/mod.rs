mod data_source;
mod location;

use crate::AppState;
use axum::Router;

pub fn make_router() -> Router<AppState> {
    Router::new()
        .nest("/location", location::make_router())
        .nest("/data-source", data_source::make_router())
}
