use axum::{extract::Path, extract::State, Json};

use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::features::notes::model::BlockType;
use crate::features::notes::model::Note;
use crate::features::notes::model::NoteBlock;
use crate::features::notes::model::NoteListComplete;
use crate::features::notes::model::NoteToShow;
use crate::features::notes::repository::NotesRepository;
use crate::AppState; // On importe notre super-erreur

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

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<NoteListComplete>>, AppError> {
    // Le miracle est ici : le "?" à la fin !
    // Si `.list()` échoue, le "?" coupe court à la fonction, attrape l'AppError, la lance à Axum
    // Axum l'attrape, lit le `IntoResponse` qu'on a codé tout à l'heure, et crie "500 Internal Server Error" au client, tout seul !
    // Si tout se passe bien, on continue.
    let notes = NotesRepository::list_notes(&state.db).await?;

    Ok(Json(notes))
}

pub async fn get_by_id(
    Path(id_note): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<NoteToShow>, AppError> {
    let _note = NotesRepository::get_note_by_id(&state.db, id_note).await?;
    Ok(Json(_note))
}

pub async fn create(
    State(state): State<crate::db::AppState>, // On prend le global state
    Json(create_note_payload): Json<CreateNotePayload>, // Axum extraie et valide automagiquement ton JSON
) -> Result<Json<Note>, AppError> {
    // On n'oublie pas le Result !

    // Hop, on passe le pool DB, et le payload au Repo.
    let id = NotesRepository::create_note(&state.db, &create_note_payload).await?;
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
}
