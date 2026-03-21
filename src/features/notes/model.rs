use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub title: String,
    pub subtitle: Option<String>,
    pub id_language: Uuid,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteBlock {
    pub id_note_block: Uuid,
    pub id_note: Uuid,
    pub block_type: String,
    pub content: String,
    pub order_index: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteList {
	pub id: Uuid,
	pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}