// La LOGIQUE métier
use axum::{
    Extension, Json,
    extract::Path,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::features::notes::model::Note;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub content: String,
}

pub async fn list(Extension(pool): Extension<MySqlPool>) -> Json<Vec<Note>> {
    let _ = pool;
    Json(vec![])
}

pub async fn get_by_id(Path(id): Path<Uuid>, Extension(pool): Extension<MySqlPool>) -> Json<Note> {
    let _ = pool;
    Json(Note {
        id,
        title: "Placeholder note".to_string(),
        content: "Implement DB query in handlers/notes.rs".to_string(),
    })
}

pub async fn create(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<CreateNotePayload>,
) -> Json<Note> {
    let _ = pool;
    Json(Note {
        id: Uuid::new_v4(),
        title: payload.title,
        content: payload.content,
    })
}