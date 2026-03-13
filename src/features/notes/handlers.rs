// La LOGIQUE métier
use axum::{
    Json,
    extract::Path,
    extract::State,
};

use serde::Deserialize;
use crate::features::notes::model::Note;
use uuid::Uuid;
use crate::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub content: String,
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Note>> {
    // on l'utilise en lisant explicitement la propriété de l'état :
    println!("La base de donnée a max de {} connexions actives", state.db.options().get_max_connections());
    
    // Suite factice 
    Json(vec![])
}

pub async fn get_by_id(Path(id): Path<Uuid>, State(db): State<crate::app_state::AppState>) -> Json<Note> {
    let _ = db;
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