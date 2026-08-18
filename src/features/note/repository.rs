use sqlx::PgPool;
use uuid::Uuid;

use crate::features::note::model::{NewNote, Note, NoteToList};
use crate::features::note::model_joined::NoteDetail;

#[derive(Clone)]
pub struct PostgresNoteRepository {
    pub pool: PgPool,
}

impl NoteRepository for PostgresNoteRepository {
    fn list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send {
        async move {
            let notes = sqlx::query_as!(
                NoteToList,
                r#"
            SELECT id_note AS "id: uuid::Uuid", title, subtitle, id_folder
            FROM notes
            "#,
            )
            .fetch_all(&self.pool)
            .await?;
            Ok(notes)
        }
    }

    /*  fn create(
        &self,
        new_note: NewNote,
    ) -> impl std::future::Future<Output = Result<Note, sqlx::Error>> + Send {
        async move {
            let id_note = Uuid::new_v4();
            let mut tx = &self.pool.begin().await?;
            let note = sqlx::query!(
                r#"
            INSERT INTO notes (id_note, note_title, note_subtitle, note_slug, note_id_folder)
            VALUES ($1, $2, $3, $4)
            "#,
                id_note,
                new_note.title,
                new_note.subtitle,
                new_note.slug,
                new_note.id_folder
            )
            .execute(&mut *tx)
            .await?;

            tx.commit().await?; // On commit la transaction, tout est validé en DB

            Ok(note)
        }
    } */

    /*     async fn create_note_block(
        e: impl Executor<'_, Database = sqlx::Postgres>,
        id_note: Uuid,
        block: &CreateNoteBlockPayload,
    ) -> Result<(), AppError> {
        let id_note_block = uuid::Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            id_note_block,
            id_note,
            block.block_type as BlockType, // Assure-toi que le type de block est correctement converti pour la DB
            block.content,
            block.order_index,
            block.metadata.as_ref().map(|m| m) // Convertit le JSON en String pour la DB
        )
        .execute(e)
        .await?;
        Ok(())
    }

    pub async fn get_note_by_id(db: &PgPool, id_note: Uuid) -> Result<NoteToShow, AppError> {
        let note = sqlx::query_as!(
            NoteSummary,
            r#"
            SELECT n.id_note AS "id_note: uuid::Uuid", n.note_title, n.note_subtitle, f.folder_name AS folder, n.created_at, n.updated_at
            FROM notes n
            JOIN folders f ON n.note_id_folder = f.id_folder
            WHERE n.id_note = $1
            "#,
            id_note
        );
        let note = match note.fetch_one(db).await {
            Ok(note) => note,
            Err(sqlx::Error::RowNotFound) => {
                return Err(AppError::NotFound(format!(
                    "Note avec id {} non trouvée",
                    id_note
                )))
            }
            Err(e) => return Err(AppError::DatabaseError(e)),
        };

        let blocks = sqlx::query_as!(
            NoteBlock,
            r#"
            SELECT id_note_block AS "id_note_block: uuid::Uuid", id_note AS "id_note: uuid::Uuid", block_type AS "block_type: BlockType", content, order_index, metadata AS "metadata: serde_json::Value"
            FROM notes_blocks
            WHERE id_note = $1
            ORDER BY order_index ASC
            "#,
            id_note
        )
        .fetch_all(db)
        .await?;

        Ok(NoteToShow {
            id_note: note.id_note,
            note_title: note.note_title,
            note_subtitle: note.note_subtitle,
            folder: note.folder,
            blocks: blocks,
            created_at: note.created_at,
            updated_at: note.updated_at,
        })
    } */
}

pub trait NoteRepository {
    fn list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send;
    /*fn create(
        &self,
        new_note: NewNote,
    ) -> impl std::future::Future<Output = Result<Vec<NoteDetail>, sqlx::Error>> + Send;
     fn delete(
        &self,
        note: DeleteNote,
    ) -> impl std::future::Future<Output = Result<Vec<NoteDetail>, sqlx::Error>> + Send; */
}
