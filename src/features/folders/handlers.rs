use axum::{
    Json,
    extract::State,
};


use crate::features::folders::model::FolderTree;
use crate::features::folders::repository::FoldersRepository;
use crate::app_state::AppState;

pub async fn get_folder_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<FolderTree>>, crate::error::AppError> {
    let folder_tree = FoldersRepository::get_folder_tree(&state.db).await?;
    Ok(Json(folder_tree))
}