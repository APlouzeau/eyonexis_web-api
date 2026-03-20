// Juste le MAPPING route → handler
use axum::{
    Router,
    routing::{get, post},
};
use crate::features::notes::handlers;

pub fn router() -> Router<crate::app_state::AppState> {
    Router::new()
        .route("/notes", get(handlers::list))
        .route("/notes/{id}", get(handlers::get_by_id))
        .route("/create-notes", post(handlers::create))
}