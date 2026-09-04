use axum::{extract::State, Json};
use uuid::Uuid;

use crate::error::AppError;
use crate::features::auth::authenticate_writer::ExtractAuthToken;
use crate::features::folder::model::{CreateFolderPayload, FolderBranch, FolderContent};
use crate::AppState;

pub async fn create(
    State(state): State<AppState>,
    _extract_auth_token: ExtractAuthToken,
    Json(create_folder_payload): Json<CreateFolderPayload>,
) -> Result<Json<FolderBranch>, AppError> {
    let new_folder = state.folder_service.create(&create_folder_payload).await?;
    Ok(Json(new_folder))
}

pub async fn folder_content(
    State(state): State<AppState>,
    _extract_auth_token: ExtractAuthToken,
    Json(get_folder_content_payload): Json<Uuid>,
) -> Result<Json<Vec<FolderContent>>, AppError> {
    let folder_content = state
        .folder_service
        .get_folder_content(&get_folder_content_payload)
        .await?;
    Ok(Json(folder_content))
}
