use sqlx::MySqlPool;
use crate::features::notes::model::Note;

pub struct NotesRepository;

impl NotesRepository {
    pub async fn list(pool: &MySqlPool) -> Vec<Note> {
        vec![]
    }
}