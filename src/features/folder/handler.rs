use std::collections::HashMap;

use axum::{extract::State, Json};
use uuid::Uuid;

use super::model::FolderNode;
use crate::error::AppError;
use crate::features::folder::model::FolderBranch;
use crate::AppState;

pub async fn get_folder_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<FolderNode>>, AppError> {
    let folder_tree = state.folder_service.get_folder_tree().await?;
    Ok(Json(folder_tree))
}
