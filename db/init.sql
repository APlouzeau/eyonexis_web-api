-- Base de données Eyonexis Knowledge Base
-- Schéma pour notes structurées avec blocks et métadonnées

create type block as enum ('text', 'code', 'heading', 'note', 'list');

-- Table des langages de programmation
CREATE TABLE IF NOT EXISTS folders(
   id_folder UUID PRIMARY KEY,
   folder_name VARCHAR(50) NOT NULL UNIQUE,
   folder_slug VARCHAR(50) NOT NULL UNIQUE,
   parent_id UUID NULL,
   created_at TIMESTAMP DEFAULT NOW(),
   FOREIGN KEY(parent_id) REFERENCES folders(id_folder) ON DELETE CASCADE
);

-- Table des tags (hooks, performance, memoization, etc.)
CREATE TABLE IF NOT EXISTS tags(
   id_tag UUID PRIMARY KEY,
   tag_name VARCHAR(50) NOT NULL UNIQUE,
   tag_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMP DEFAULT NOW()
);

-- Table principale des notes/articles
CREATE TABLE IF NOT EXISTS notes(
   id_note UUID PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   subtitle VARCHAR(500),
   id_folder UUID NOT NULL,
   created_at TIMESTAMP DEFAULT NOW(),
   updated_at TIMESTAMP DEFAULT NOW(),
   FOREIGN KEY(id_folder) REFERENCES folders(id_folder)
);

   CREATE INDEX idx_notes_folder ON notes (id_folder);
   CREATE INDEX idx_notes_updated ON notes (updated_at DESC);

-- Types de blocs possibles
-- text: paragraphe normal
-- code: bloc de code avec syntaxe
-- heading: titre de section
-- note: note personnelle
-- list: liste à puces
CREATE TABLE IF NOT EXISTS notes_blocks(
   id_note_block UUID PRIMARY KEY,
   id_note UUID NOT NULL,
   block_type block,
   content TEXT NOT NULL,
   order_index INT NOT NULL,
   metadata JSON,  -- Pour stocker le langage du code, niveau de heading, etc.
   created_at TIMESTAMP DEFAULT NOW(),
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE
);

   CREATE INDEX idx_blocks_note ON notes_blocks (id_note);
   CREATE INDEX idx_blocks_order ON notes_blocks (id_note, order_index);

-- Relation many-to-many: notes <-> tags
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


-- Données de test
INSERT INTO folders (id_folder, parent_id, folder_name, folder_slug) VALUES
('550e8400-5440-0000-0000-000000000001', NULL, 'JavaScript', 'javascript'),
('550e8400-5440-0000-0000-000000000002', NULL, 'TypeScript', 'typescript'),
('550e8400-5440-0000-0000-000000000003', NULL, 'Python', 'python'),
('550e8400-5440-0000-0000-000000000004', NULL, 'Rust', 'rust'),
('550e8400-5440-0000-0000-000000000005', NULL, 'React', 'react');

INSERT INTO folders (id_folder, parent_id, folder_name, folder_slug) VALUES
('650e8400-5440-0000-0000-000000000001', '550e8400-5440-0000-0000-000000000004', 'Fondamentaux', 'fondamentaux'),
('650e8400-5440-0000-0000-000000000002', NULL, 'Design Patterns', 'design-patterns'),
('650e8400-5440-0000-0000-000000000003', NULL, 'Algorithmes', 'algorithmes'),
('650e8400-5440-0000-0000-000000000004', '550e8400-5440-0000-0000-000000000004', 'API & Web', 'api-web');

INSERT INTO tags (id_tag, tag_name, tag_slug) VALUES
('750e8400-5440-0000-0000-000000000001', 'hooks', 'hooks'),
('750e8400-5440-0000-0000-000000000002', 'performance', 'performance'),
('750e8400-5440-0000-0000-000000000003', 'memoization', 'memoization');
