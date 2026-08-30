use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::handlers::{read, write};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/folder-tree", get(read::get_folder_tree))
        .route("/create", post(write::create))
}
