use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::folder::NoteToList;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderNode {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub children: Vec<FolderNode>,
    pub notes: Vec<NoteToList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderBranch {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub parent_id: Option<Uuid>,
}
