use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id_note: Uuid,
	pub title: String,
    pub subtitle: Option<String>,
    pub id_folder: Uuid,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteBlock {
    pub id_note_block: Uuid,
    pub id_note: Uuid,
    pub block_type: BlockType,
    pub content: String,
    pub order_index: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteListComplete {
	pub id: Uuid,
	pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteListTree {
    pub id: Uuid,
    pub title: String,
    pub folder_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteToShow {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub folder: String,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NoteSummary {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub folder: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, Copy)]
#[sqlx(type_name = "block", rename_all = "lowercase")]
pub enum BlockType {
    Text,
    Code,
    Heading,
    Note,
    List,
}