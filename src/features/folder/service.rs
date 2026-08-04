use super::model::{DeleteFolder, NewFolder};
use super::model_response::FolderToListResponse;
use super::repository::FolderRepository;

#[derive(Clone)]
pub struct FolderService<R: FolderRepository> {
    pub repository: R,
}

impl<R: FolderRepository> FolderService<R> {
    pub async fn list(&self) -> Result<Vec<FolderToListResponse>, sqlx::Error> {
        let folders = self.repository.list().await?;
        Ok(folders
            .into_iter()
            .map(FolderToListResponse::from)
            .collect())
    }

    pub async fn create(&self, new_folder: NewFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.create(new_folder).await?;
        self.get_all().await
    }

    pub async fn delete(&self, id: DeleteFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    }
}
