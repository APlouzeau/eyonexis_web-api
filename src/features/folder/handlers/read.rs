use axum::{extract::State, Json};

use crate::error::AppError;
use crate::features::folder::model::FolderNode;
use crate::AppState;

pub async fn get_folder_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<FolderNode>>, AppError> {
    let folder_tree = state.folder_service.get_folder_tree().await?;
    Ok(Json(folder_tree))
}
