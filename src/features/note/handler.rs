use crate::features::note::model::{NoteToList, NoteToShow};
use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::{debug_handler, debug_middleware};

use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::AppState;

use crate::features::note::model::BlockType;

#[derive(Debug, Deserialize)]
pub struct CreateNoteBlockPayload {
    pub block_type: BlockType,
    pub content: String,
    pub order_index: i32,
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub subtitle: Option<String>,
    pub id_folder: Uuid,
    pub blocks: Vec<CreateNoteBlockPayload>,
}

#[debug_handler]
pub async fn get_notes_by_folder_id(
    State(state): State<AppState>,
    Path(id_folder): Path<Uuid>,
) -> Result<Json<Vec<NoteToList>>, AppError> {
    let notes = state.note_service.list_by_folder(id_folder).await?;
    Ok(Json(notes))
}

pub async fn get_note_by_id(
    State(state): State<AppState>,
    Path(id_note): Path<Uuid>,
) -> Result<Json<NoteToShow>, AppError> {
    let note = state.note_service.get_note_by_id(id_note).await?;
    Ok(Json(note))
}

/*
pub async fn create(
    State(state): State<crate::db::AppState>, // On prend le global state
    Json(create_note_payload): Json<CreateNotePayload>, // Axum extraie et valide automagiquement ton JSON
) -> Result<Json<Note>, AppError> {
    // On n'oublie pas le Result !

    // Hop, on passe le pool DB, et le payload au Repo.
    let id = NoteRepository::create_note(&state.db, &create_note_payload).await?;
    let slug = create_note_payload.title.to_lowercase().replace(" ", "-");

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
    }))
}*/
