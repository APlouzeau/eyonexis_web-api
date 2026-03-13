// La LOGIQUE métier
use axum::{
    Json,
    extract::Path,
    extract::State,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::features::notes::model::Note;
use crate::features::notes::repository::NotesRepository;
use crate::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub content: String,
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = NotesRepository::list(&state.db).await;
    Json(notes)
}

pub async fn get_by_id(Path(id): Path<Uuid>, State(state): State<crate::app_state::AppState>) -> Json<Note> {
    let _ = state;
    Json(Note {
        id,
        title: "Placeholder note".to_string(),
        content: "Implement DB query in handlers/notes.rs".to_string(),
    })
}

pub async fn create(
    State(db): State<crate::app_state::AppState>,
    Json(payload): Json<CreateNotePayload>,
) -> Json<Note> {
    let _ = db;
    Json(Note {
        id: Uuid::new_v4(),
        title: payload.title,
        content: payload.content,
    })
}