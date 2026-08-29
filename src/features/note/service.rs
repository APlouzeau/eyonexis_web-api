use uuid::Uuid;

use crate::features::note::model::{CreateNotePayload, NewNote, NoteToList, NoteToShow};

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

    pub async fn get_note_by_id(&self, id_note: Uuid) -> Result<NoteToShow, sqlx::Error> {
        let note = self.repository.get_note_by_id(id_note).await?;
        Ok(note)
    }

    pub async fn create(&self, new_note: CreateNotePayload) -> Result<(), sqlx::Error> {
        let id_new_note = NewNote {
            id_note: Uuid::new_v4(),
        };
        self.repository
            .create_full_note(id_new_note, &new_note)
            .await?;

        Ok(())
    }

    /*     pub async fn delete(&self, id: DeleteNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}
