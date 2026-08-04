use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::features::note::model::NoteListTree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderTree {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub children: Vec<FolderTree>,
    pub notes: Vec<NoteListTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct FolderRow {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub parent_id: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub note_title: Option<String>,
    pub note_id_folder: Uuid,
    pub note_slug: Option<String>,
}