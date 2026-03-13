use sqlx::MySqlPool;
use crate::features::notes::model::Note;
use crate::error::AppError; // On importe notre nouvelle super-erreur

pub struct NotesRepository;

impl NotesRepository {
    pub async fn list(pool: &MySqlPool) -> Result<Vec<Note>, AppError> {
        // Le `Ok(...)` englobe notre résultat pour dire "Pas d'erreur ici"
        // Le jour où on fera `sqlx::query!().fetch_all(pool).await?`, c'est le `?` qui renverra l'AppError en cas de crash
        Ok(vec![])
    }

    pub async fn create_note(pool: &MySqlPool, title: &str, content: &str) -> Result<String, AppError> {
        let id_note = uuid::Uuid::new_v4().to_string(); // 'id_note' fait 36 char, top
    
        // On hardcode (pour l'exercice) que c'est une note Rust (id venant de ton init.sql)
        let lang_rust_id = "550e8400-e29b-41d4-a716-446655440004"; 

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