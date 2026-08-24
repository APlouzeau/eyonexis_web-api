use sqlx::PgPool;
use uuid::Uuid;

use crate::features::note::model::{NoteBlock, NoteSummary, NoteToList, NoteToShow, BlockType};

#[derive(Clone)]
pub struct PostgresNoteRepository {
    pub pool: PgPool,
}

impl NoteRepository for PostgresNoteRepository {
    fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send {
        async move {
            let notes = sqlx::query_as!(
                NoteToList,
                r#"
            SELECT id_note AS "id: uuid::Uuid", title, subtitle
            FROM notes
            WHERE id_folder = $1
            "#,
                id_folder
            )
            .fetch_all(&self.pool)
            .await?;
            Ok(notes)
        }
    }

    fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> impl std::future::Future<Output = Result<NoteToShow, sqlx::Error>> + Send {
        async move {
            let note = sqlx::query_as!(
                NoteSummary,
                r#"
            SELECT n.id_note AS "id_note: uuid::Uuid", n.title, n.subtitle, n.created_at, n.updated_at, f.folder_name as folder, n.slug
            FROM notes n
            INNER JOIN folders f ON n.id_folder = f.id_folder
            WHERE n.id_note = $1
            "#,
                id_note
            )            
            .fetch_optional(&self.pool)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

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
        .fetch_all(&self.pool)
        .await?;

            Ok(NoteToShow {
                id_note: note.id_note,
                title: note.title,
                subtitle: note.subtitle,
                folder: note.folder,
                slug : note.slug,
                blocks: blocks,
                created_at: note.created_at,
                updated_at: note.updated_at,
            })
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
    } */
}

pub trait NoteRepository {
    fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send;
    fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> impl std::future::Future<Output = Result<NoteToShow, sqlx::Error>> + Send;
    /*fn create(
        &self,
        new_note: NewNote,
    ) -> impl std::future::Future<Output = Result<Vec<NoteDetail>, sqlx::Error>> + Send;
     fn delete(
        &self,
        note: DeleteNote,
    ) -> impl std::future::Future<Output = Result<Vec<NoteDetail>, sqlx::Error>> + Send; */
}
