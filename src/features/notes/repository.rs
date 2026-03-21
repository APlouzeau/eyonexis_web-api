use sqlx::Executor;
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::features::notes::model::NoteSummary;
use crate::features::notes::model::NoteBlock;
use crate::features::notes::model::NoteList;
use crate::features::notes::handlers::CreateNotePayload;
use crate::features::notes::handlers::CreateNoteBlockPayload;
use crate::error::AppError;
use crate::features::notes::model::NoteToShow; // On importe notre nouvelle super-erreur

pub struct NotesRepository;

impl NotesRepository {
    pub async fn list(pool: &MySqlPool) -> Result<Vec<NoteList>, AppError> {
        let notes = sqlx::query_as!(
            NoteList,
            r#"
            SELECT id_note AS `id: uuid::Uuid`, title, created_at, updated_at
            FROM notes
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(notes)
    }

    pub async fn create_note(pool: &MySqlPool, create_note_payload: &CreateNotePayload) -> Result<Uuid, AppError> {
        let id_note = uuid::Uuid::new_v4(); // 'id_note' fait 36 char, top
        let mut tx = pool.begin().await?; // On démarre une transaction

        // ATTENTION: La macro va valider ça sur ton MariaDB !
        // Si ta table `notes` prend un sous-titre optionnel, il faut l'ajouter (ici je suppose que `subtitle` n'est pas encore dans la table si elle est basique, mais on l'ajoute si nécessaire. Attend, dans init.sql, il y a un subtitle ? Non, je vais laisser title et id_language pour le moment)
        sqlx::query!(
            r#"
            INSERT INTO notes (id_note, title, subtitle, id_language)
            VALUES (?, ?, ?, ?)
            "#,
            id_note,
            create_note_payload.title,
            create_note_payload.subtitle,
            create_note_payload.id_language
        )
        .execute(&mut *tx)
        .await?;       

            for block in &create_note_payload.blocks {
        Self::create_note_block(&mut *tx, id_note, block).await?;
        }

        tx.commit().await?; // On commit la transaction, tout est validé en DB
        // NB: Tu as créé une super table "notes_blocks" pour le "content", on l'ignorera pour cette première requête d'apprentissage, on insère juste l'en-tête de la note pour commencer !

        Ok(id_note) 
    }

    async fn create_note_block(e: impl Executor<'_,Database = sqlx::MySql>, id_note: Uuid, block: &CreateNoteBlockPayload) -> Result<(), AppError> {
         let id_note_block = uuid::Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id_note_block,
            id_note,
            block.block_type,
            block.content,
            block.order_index,
            block.metadata.as_ref().map(|m| m.to_string()) // Convertit le JSON en String pour la DB
        )
        .execute(e)
        .await?;
        Ok(())
    }

    pub async fn get_note_by_id(pool: &MySqlPool, id_note: Uuid) -> Result<NoteToShow, AppError> {
        let note = sqlx::query_as!(
            NoteSummary,
            r#"
            SELECT n.id_note AS `id_note: uuid::Uuid`, n.title, n.subtitle, l.language_name AS language, n.created_at, n.updated_at
            FROM notes n
            JOIN languages l ON n.id_language = l.id_language
            WHERE n.id_note = ?
            "#,
            id_note
        )
        .fetch_one(pool)
        .await?;

        let blocks = sqlx::query_as!(
            NoteBlock,
            r#"
            SELECT id_note_block AS `id_note_block: uuid::Uuid`, id_note AS `id_note: uuid::Uuid`, block_type, content, order_index, metadata AS `metadata: serde_json::Value`
            FROM notes_blocks
            WHERE id_note = ?
            ORDER BY order_index ASC
            "#,
            id_note
        )
        .fetch_all(pool)
        .await?;

        Ok(NoteToShow {
            id_note: note.id_note,
            title: note.title,
            subtitle: note.subtitle,
            language: note.language,
            blocks: blocks,
            created_at: note.created_at,
            updated_at: note.updated_at,
        })

    }

}