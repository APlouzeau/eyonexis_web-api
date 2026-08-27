use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notes/{id_folder}", get(handler::get_notes_by_folder_id))
        .route("/note/{id_note}", get(handler::get_note_by_id))
    /*  .route("/note/{id_note}", get(handler::get_by_id))
    .route("/create-notes", post(handler::create)) */
}
