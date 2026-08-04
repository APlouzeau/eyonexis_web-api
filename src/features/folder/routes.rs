use axum::{routing::get, Router};

use super::handler;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/folder-tree", get(handler::get_folder_tree))
}
