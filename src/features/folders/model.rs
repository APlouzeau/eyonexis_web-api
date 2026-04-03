use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::features::notes::model::NoteList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderTree {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub children: Vec<FolderTree>,
    pub notes: Vec<NoteList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRow {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub parent_id: Option<Uuid>,
}