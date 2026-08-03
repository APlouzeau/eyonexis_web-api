use axum::{
    Router,
    routing::{get},
};

use crate::features::folders::handlers;

pub fn router() -> Router<crate::db::AppState> {
    Router::new()
        .route("/folder-tree", get(handlers::get_folder_tree))
}
