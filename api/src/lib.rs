#![forbid(unsafe_code)]
#![warn(clippy::correctness, clippy::pedantic, clippy::style, clippy::perf)]

mod api;
mod commons;

use axum::Router;
pub use commons::{AppState, Config, relations};

pub fn make_app() -> Router<AppState> {
    Router::new().nest("/api", api::make_router())
}
