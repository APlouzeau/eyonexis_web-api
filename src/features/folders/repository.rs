use sqlx::PgPool;
use crate::error::AppError;
use crate::features::folders::model::FolderTree;
use crate::features::folders::model::FolderRow;
use uuid::Uuid;

pub struct FoldersRepository;


impl FoldersRepository {
    pub async fn get_folder_tree(db: &PgPool) -> Result<Vec<FolderTree>, AppError> {
        
        let query_result :Vec<FolderRow> = sqlx::query_as!(
            FolderRow,
            r#"
            SELECT id_folder, folder_name, parent_id
            FROM folders
            "#
        )
        .fetch_all(db)
        .await?;

        let result = Self::build_children(None, &query_result);

        Ok(result)
    }

    pub fn build_children(parent_id: Option<Uuid>, all_folders: &Vec<FolderRow>) -> Vec<FolderTree> {
    all_folders
        .iter() // On itère sur tous les dossiers
        .filter(|f| f.parent_id == parent_id) // On garde que ceux dont le parent_id correspond à notre parent_id actuel
        .map(|f| FolderTree { // On construit un FolderTree pour chacun de ces dossiers
            children: Self::build_children(Some(f.id_folder), all_folders), // récursion
            id_folder: f.id_folder,
            folder_name: f.folder_name.clone(),
            notes: vec![], // Pas de notes pour l'instant
        })
        .collect()
}
}