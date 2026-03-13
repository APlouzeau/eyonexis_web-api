use axum::{Router, routing::get};
use crate::features::health::handlers::health;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
}