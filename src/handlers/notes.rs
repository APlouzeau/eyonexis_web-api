// La LOGIQUE métier
use axum::{Json, Extension};
use sqlx::MySqlPool;
use crate::models::Note;

pub async fn list(Extension(pool): Extension<MySqlPool>) -> Json<Vec<Note>> {
    // Requête SQL, logique, etc.
}