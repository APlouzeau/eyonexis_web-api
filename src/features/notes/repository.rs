use sqlx::MySqlPool;
use uuid::Uuid;
use crate::features::notes::model::Note;
use crate::error::AppError; // On importe notre nouvelle super-erreur

pub struct NotesRepository;

impl NotesRepository {
    pub async fn list(pool: &MySqlPool) -> Result<Vec<Note>, AppError> {
        let notes = sqlx::query_as!(
            Note,
            // Regarde le "as `id: uuid::Uuid`" ! 
            // C'est ça qui dit à la Macro : "Parse ce varchar comme un vrai UUID natif"
            r#"
            SELECT id_note AS `id: uuid::Uuid`, title, '' AS body, created_at, updated_at
            FROM notes
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(notes)
    }

    pub async fn create_note(pool: &MySqlPool, title: &str, _body: &str) -> Result<Uuid, AppError> {
        let id_note = uuid::Uuid::new_v4(); // 'id_note' fait 36 char, top
    
        // On hardcode (pour l'exercice) que c'est une note Rust (id venant de ton init.sql)
        let lang_rust_id = uuid::Uuid::parse_str("550e8400-5440-0000-0000-000000000004").unwrap(); 

        // ATTENTION: La macro va valider ça sur ton MariaDB !
        sqlx::query!(
            r#"
            INSERT INTO notes (id_note, title, id_language)
            VALUES (?, ?, ?)
            "#,
            id_note,
            title,
            lang_rust_id
        )
        .execute(pool) 
        .await?;       

        // NB: Tu as créé une super table "notes_blocks" pour le "content", on l'ignorera pour cette première requête d'apprentissage, on insère juste l'en-tête de la note pour commencer !

        Ok(id_note) 
    }
}