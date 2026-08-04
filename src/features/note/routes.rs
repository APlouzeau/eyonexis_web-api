use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notes", get(handler::get_all))
        .route("/note/{id_note}", get(handler::get_by_id))
        .route("/create-notes", post(handler::create))
}
