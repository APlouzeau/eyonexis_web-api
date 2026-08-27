use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NoteResponse {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
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
    pub order_index: u32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NewNoteResponse {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub id_folder: Uuid,
    pub blocks: Vec<NoteBlock>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteToListResponse {
    pub id: Uuid,
    pub note_title: String,
    pub note_folder_id: Uuid,
    pub note_subtitle: Option<String>,
}

/* impl From<NoteDetail> for NoteResponse {
    fn from(note_detail: NoteDetail) -> Self {
        NoteResponse {
            id_note: note_detail.id_note,
            title: note_detail.title,
            subtitle: note_detail.subtitle,
            slug: note_detail.slug,
            id_folder: note_detail.id_folder,
            blocks: note_detail.blocks,
            created_at: note_detail.created_at,
            updated_at: note_detail.update_at,
        }
    }
} */
