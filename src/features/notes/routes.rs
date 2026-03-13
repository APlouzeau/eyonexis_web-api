// Juste le MAPPING route → handler
use axum::{
    Router,
    routing::{get, post},
};
use crate::features::notes::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/notes", get(handlers::list))
        .route("/notes/:id", get(handlers::get_by_id))
        .route("/notes", post(handlers::create))
}