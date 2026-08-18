use super::handler;
use axum::{routing::get, Router};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(handler::health))
}
