use std::collections::HashSet;

use sqlx::PgPool;
use crate::error::AppError;
use crate::features::folders::model::FolderTree;
use crate::features::folders::model::FolderRow;
use crate::features::notes::model::NoteListTree;
use uuid::Uuid;

pub struct FoldersRepository;


impl FoldersRepository {
    pub async fn get_folder_tree(db: &PgPool) -> Result<Vec<FolderTree>, AppError> {
        
        let query_result :Vec<FolderRow> = sqlx::query_as!(
            FolderRow,
            r#"
            SELECT f.id_folder, f.folder_name, f.parent_id, n.id_note as "id_note: uuid::Uuid", n.title as note_title, n.id_folder as note_folder_id
            FROM folders f
            LEFT JOIN notes n ON f.id_folder = n.id_folder
            "#
        )
        .fetch_all(db)
        .await?;

        let result = Self::build_children_tree(None, &query_result);

        Ok(result)
    }

    pub fn build_children_tree(parent_id: Option<Uuid>, all_folders: &Vec<FolderRow>) -> Vec<FolderTree> {

        let mut seen: HashSet<Uuid> = HashSet::new(); // Un HashSet pour suivre les dossiers déjà vus

    all_folders
        .iter() // On itère sur tous les dossiers
        .filter(|f| seen.insert(f.id_folder) && f.parent_id == parent_id) // On garde que ceux dont le parent_id correspond à notre parent_id actuel
        .map(|f| FolderTree { // On construit un FolderTree pour chacun de ces dossiers
            children: Self::build_children_tree(Some(f.id_folder), all_folders), // récursion
            id_folder: f.id_folder,
            folder_name: f.folder_name.clone(),
            notes: Self::build_folder_notes(f.id_folder, all_folders),
        })
        .collect()
}

    pub fn build_folder_notes(folder_id: Uuid, all_folders: &Vec<FolderRow>) -> Vec<NoteListTree> {
        all_folders
            .iter()
            .filter(|n| n.id_folder == folder_id)
            .filter_map(|n| {
                Some(NoteListTree {
                    id: n.id_note?,
                    title: n.note_title.clone()?,
                    folder_id: n.id_folder,
                })
            })
            .collect()
    }
}