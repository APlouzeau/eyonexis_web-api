use axum::extract::State;
use axum::Json;

use crate::features::auth::authenticate_writer::ExtractAuthToken;
use crate::features::note::model::CreateNotePayload;

use crate::error::AppError;
use crate::AppState;

pub struct NoteCreated;

pub async fn create(
    State(state): State<AppState>,
    _extract_auth_token: ExtractAuthToken,
    Json(create_note_payload): Json<CreateNotePayload>,
) -> Result<(), AppError> {
    state.note_service.create(&create_note_payload).await?;
    Ok(())
}
