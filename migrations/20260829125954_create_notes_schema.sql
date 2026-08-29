create type block as enum ('text', 'code', 'heading', 'tip');

CREATE TABLE IF NOT EXISTS folders(
   id_folder UUID PRIMARY KEY,
   folder_name VARCHAR(50) NOT NULL UNIQUE,
   folder_slug VARCHAR(50) NOT NULL UNIQUE,
   parent_id UUID NULL,
   created_at TIMESTAMPTZ NOT NULL  DEFAULT NOW(),
   FOREIGN KEY(parent_id) REFERENCES folders(id_folder) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS device_tokens(
    id_device SERIAL PRIMARY KEY,
    device VARCHAR(50) NOT NULL, 
    token_hash CHAR(64) NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL  DEFAULT NOW(), 
    last_connected_at TIMESTAMPTZ NULL
);

CREATE TABLE IF NOT EXISTS tags(
   id_tag UUID PRIMARY KEY,
   tag_name VARCHAR(50) NOT NULL UNIQUE,
   tag_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notes(
   id_note UUID PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   slug VARCHAR(255) NOT NULL UNIQUE,
   subtitle VARCHAR(500),
   id_folder UUID NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   FOREIGN KEY(id_folder) REFERENCES folders(id_folder)
);

   CREATE INDEX idx_notes_folder ON notes (id_folder);
   CREATE INDEX idx_notes_updated ON notes (updated_at DESC);

   CREATE TABLE IF NOT EXISTS notes_blocks(
   id_note_block UUID PRIMARY KEY,
   id_note UUID NOT NULL,
   block_type block NOT NULL,
   content TEXT NOT NULL,
   order_index INT NOT NULL,
   metadata JSON,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE
);

   CREATE INDEX idx_blocks_note ON notes_blocks (id_note);
   CREATE INDEX idx_blocks_order ON notes_blocks (id_note, order_index);

CREATE TABLE IF NOT EXISTS note_tags(
   id_note UUID,
   id_tag UUID,
   PRIMARY KEY(id_note, id_tag),
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE,
   FOREIGN KEY(id_tag) REFERENCES tags(id_tag) ON DELETE CASCADE
);

CREATE INDEX idx_note_tags_note ON note_tags (id_note);
CREATE INDEX idx_note_tags_tag ON note_tags (id_tag);

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_notes_updated_at
BEFORE UPDATE ON notes
FOR EACH ROW EXECUTE FUNCTION set_updated_at();