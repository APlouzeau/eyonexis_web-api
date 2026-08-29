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
) -> Result<NoteCreated, AppError> {
    Ok(NoteCreated)
}

/*    let slug = create_note_payload.title.to_lowercase().replace(" ", "-");

let mapped_blocks = create_note_payload
    .blocks
    .into_iter()
    .map(|b| NoteBlock {
        id_note_block: Uuid::new_v4(),
        id_note: id,
        block_type: b.block_type,
        content: b.content,
        order_index: b.order_index,
        metadata: b.metadata,
    })
    .collect(); // collect() rassemble tout dans un Vec<NoteBlock>
                // On renvoie un object propre en réponse au client !
Ok(Json(Note {
    id_note: id,
    title: create_note_payload.title,
    subtitle: create_note_payload.subtitle,
    slug,
    id_folder: create_note_payload.id_folder,
    blocks: mapped_blocks,
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
}))*/
