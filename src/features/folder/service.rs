use crate::features::folder::model::FolderTree;

use super::repository::FolderRepository;

#[derive(Clone)]
pub struct FolderService<R: FolderRepository> {
    pub repository: R,
}

impl<R: FolderRepository> FolderService<R> {
    pub async fn get_folder_tree(&self) -> Result<Vec<FolderTree>, sqlx::Error> {
        let folders = self.repository.get_folder_tree().await?;
        Ok(folders.into_iter().map(FolderTree::from).collect())
    }

    /*     pub async fn create(&self, new_folder: NewFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.create(new_folder).await?;
        self.get_all().await
    }

    pub async fn delete(&self, id: DeleteFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}
