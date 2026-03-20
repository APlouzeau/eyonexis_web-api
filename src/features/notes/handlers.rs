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
use crate::error::AppError; // On importe notre super-erreur

#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub body: String,
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Note>>, AppError> {
    
    // Le miracle est ici : le "?" à la fin !
    // Si `.list()` échoue, le "?" coupe court à la fonction, attrape l'AppError, la lance à Axum
    // Axum l'attrape, lit le `IntoResponse` qu'on a codé tout à l'heure, et crie "500 Internal Server Error" au client, tout seul !
    // Si tout se passe bien, on continue.
    let notes = NotesRepository::list(&state.db).await?;
    
    Ok(Json(notes))
}

pub async fn get_by_id(Path(id): Path<Uuid>, State(state): State<crate::app_state::AppState>) -> Json<Note> {
    let _ = state;
    Json(Note {
        id,
        title: "Placeholder note".to_string(),
        body: "Placeholder content".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

pub async fn create(
    State(state): State<crate::app_state::AppState>, // On prend le global state
    Json(payload): Json<CreateNotePayload>, // Axum extraie et valide automagiquement ton JSON
) -> Result<Json<Note>, AppError> { // On n'oublie pas le Result !
    
    // Hop, on passe le pool DB, et les strings au Repo.
    // Le `?` gère l'échec tout seul. Si succès, 'id' aura la valeur String générée
    let id = NotesRepository::create_note(&state.db, &payload.title, &payload.body).await?;

    // On renvoie un object propre en réponse au client !
    Ok(Json(Note {
        id,
        title: payload.title,
        body: payload.body,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }))
}