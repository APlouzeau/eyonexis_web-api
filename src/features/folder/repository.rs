use sqlx::PgPool;

use super::model::FolderBranch;

#[derive(Clone)]
pub struct PostgresFolderRepository {
    pub pool: PgPool,
}

impl FolderRepository for PostgresFolderRepository {
    async fn get_folder_tree(&self) -> Result<Vec<FolderBranch>, sqlx::Error> {
        let result = sqlx::query_as!(
            FolderBranch,
            r#"
            SELECT f.id_folder as "id_folder: uuid::Uuid", f.folder_name, f.parent_id as "parent_id: uuid::Uuid"
            FROM folders f
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

pub trait FolderRepository {
    fn get_folder_tree(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<FolderBranch>, sqlx::Error>> + Send;
}
