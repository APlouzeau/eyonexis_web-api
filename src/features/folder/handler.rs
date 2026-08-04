use axum::{extract::State, Json};

use super::model::FolderTree;
use crate::AppState;

pub async fn get_folder_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<FolderTree>>, crate::error::AppError> {
    let folder_tree = state.folder_service.get_folder_tree().await?;
    Ok(Json(folder_tree))
}
