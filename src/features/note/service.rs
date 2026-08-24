use uuid::Uuid;

use crate::features::note::model::NoteToList;

use super::repository::NoteRepository;

#[derive(Clone)]
pub struct NoteService<R: NoteRepository> {
    pub repository: R,
}

impl<R: NoteRepository> NoteService<R> {
    pub async fn list_by_folder(&self, id_folder: Uuid) -> Result<Vec<NoteToList>, sqlx::Error> {
        let notes = self.repository.list_by_folder(id_folder).await?;
        Ok(notes.into_iter().map(NoteToList::from).collect())
    }

    /*     pub async fn create(&self, new_note: NewNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.create(new_note).await?;
        self.get_all().await
    }

    pub async fn delete(&self, id: DeleteNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}
