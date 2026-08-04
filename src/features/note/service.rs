use super::model::{DeleteNote, NewNote};
use super::model_response::NoteToListResponse;
use super::repository::NoteRepository;

#[derive(Clone)]
pub struct NoteService<R: NoteRepository> {
    pub repository: R,
}

impl<R: NoteRepository> NoteService<R> {
    pub async fn list(&self) -> Result<Vec<NoteToListResponse>, sqlx::Error> {
        let notes = self.repository.list().await?;
        Ok(notes.into_iter().map(NoteToListResponse::from).collect())
    }

    pub async fn create(&self, new_note: NewNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.create(new_note).await?;
        self.get_all().await
    }

    pub async fn delete(&self, id: DeleteNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    }
}
