use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderTree {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub children: Vec<Uuid>,
}
