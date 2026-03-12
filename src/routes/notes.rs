// Juste le MAPPING route → handler
use axum::{Router, routing::get};
use crate::handlers::notes;

pub fn router() -> Router {
    Router::new()
        .route("/notes", get(notes::list))
        .route("/notes/:id", get(notes::get_by_id))
        .route("/notes", post(notes::create))
}