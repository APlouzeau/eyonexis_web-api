use axum::{extract::State, Json};

use crate::error::AppError;
use crate::features::auth::authenticate_writer::ExtractAuthToken;
use crate::features::folder::model::{CreateFolderPayload, FolderBranch};
use crate::AppState;

pub async fn create(
    State(state): State<AppState>,
    _extract_auth_token: ExtractAuthToken,
    Json(create_folder_payload): Json<CreateFolderPayload>,
) -> Result<Json<FolderBranch>, AppError> {
    let new_folder = state.folder_service.create(&create_folder_payload).await?;
    Ok(Json(new_folder))
}
