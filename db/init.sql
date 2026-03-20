-- Base de données Eyonexis Knowledge Base
-- Schéma pour notes structurées avec blocks et métadonnées

-- Table des langages de programmation
CREATE TABLE IF NOT EXISTS languages(
   id_language BINARY(16) PRIMARY KEY,
   language_name VARCHAR(50) NOT NULL UNIQUE,
   language_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table des catégories (Fondamentaux, Design Patterns, etc.)
CREATE TABLE IF NOT EXISTS categories(
   id_category BINARY(16) PRIMARY KEY,
   category_name VARCHAR(50) NOT NULL UNIQUE,
   category_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table des tags (hooks, performance, memoization, etc.)
CREATE TABLE IF NOT EXISTS tags(
   id_tag BINARY(16) PRIMARY KEY,
   tag_name VARCHAR(50) NOT NULL UNIQUE,
   tag_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table principale des notes/articles
CREATE TABLE IF NOT EXISTS notes(
   id_note BINARY(16) PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   subtitle VARCHAR(500),
   id_language BINARY(16) NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
   FOREIGN KEY(id_language) REFERENCES languages(id_language),
   INDEX idx_notes_language (id_language),
   INDEX idx_notes_updated (updated_at DESC)
);

-- Types de blocs possibles
-- text: paragraphe normal
-- code: bloc de code avec syntaxe
-- heading: titre de section
-- note: note personnelle
-- list: liste à puces
CREATE TABLE IF NOT EXISTS notes_blocks(
   id_note_block BINARY(16) PRIMARY KEY,
   id_note BINARY(16) NOT NULL,
   block_type ENUM('text', 'code', 'heading', 'note', 'list') NOT NULL,
   content TEXT NOT NULL,
   order_index INT NOT NULL,
   metadata JSON,  -- Pour stocker le langage du code, niveau de heading, etc.
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE,
   INDEX idx_blocks_note (id_note),
   INDEX idx_blocks_order (id_note, order_index)
);

-- Relation many-to-many: notes <-> tags
CREATE TABLE IF NOT EXISTS note_tags(
   id_note BINARY(16),
   id_tag BINARY(16),
   PRIMARY KEY(id_note, id_tag),
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE,
   FOREIGN KEY(id_tag) REFERENCES tags(id_tag) ON DELETE CASCADE,
   INDEX idx_note_tags_note (id_note),
   INDEX idx_note_tags_tag (id_tag)
);

-- Relation many-to-many: notes <-> categories
CREATE TABLE IF NOT EXISTS note_categories(
   id_note BINARY(16),
   id_category BINARY(16),
   PRIMARY KEY(id_note, id_category),
   FOREIGN KEY(id_note) REFERENCES notes(id_note) ON DELETE CASCADE,
   FOREIGN KEY(id_category) REFERENCES categories(id_category) ON DELETE CASCADE,
   INDEX idx_note_categories_note (id_note),
   INDEX idx_note_categories_category (id_category)
);

-- Données de test
INSERT INTO languages (id_language, language_name, language_slug) VALUES
(UNHEX(REPLACE('550e8400-5440-0000-0000-000000000001', '-', '')), 'JavaScript', 'javascript'),
(UNHEX(REPLACE('550e8400-5440-0000-0000-000000000002', '-', '')), 'TypeScript', 'typescript'),
(UNHEX(REPLACE('550e8400-5440-0000-0000-000000000003', '-', '')), 'Python', 'python'),
(UNHEX(REPLACE('550e8400-5440-0000-0000-000000000004', '-', '')), 'Rust', 'rust'),
(UNHEX(REPLACE('550e8400-5440-0000-0000-000000000005', '-', '')), 'React', 'react');

INSERT INTO categories (id_category, category_name, category_slug) VALUES
(UNHEX(REPLACE('650e8400-5440-0000-0000-000000000001', '-', '')), 'Fondamentaux', 'fondamentaux'),
(UNHEX(REPLACE('650e8400-5440-0000-0000-000000000002', '-', '')), 'Design Patterns', 'design-patterns'),
(UNHEX(REPLACE('650e8400-5440-0000-0000-000000000003', '-', '')), 'Algorithmes', 'algorithmes'),
(UNHEX(REPLACE('650e8400-5440-0000-0000-000000000004', '-', '')), 'API & Web', 'api-web');

INSERT INTO tags (id_tag, tag_name, tag_slug) VALUES
(UNHEX(REPLACE('750e8400-5440-0000-0000-000000000001', '-', '')), 'hooks', 'hooks'),
(UNHEX(REPLACE('750e8400-5440-0000-0000-000000000002', '-', '')), 'performance', 'performance'),
(UNHEX(REPLACE('750e8400-5440-0000-0000-000000000003', '-', '')), 'memoization', 'memoization');
