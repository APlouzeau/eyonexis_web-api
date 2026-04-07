-- Base de données Eyonexis Knowledge Base
-- Schéma pour notes structurées avec blocks et métadonnées

create type block as enum ('text', 'code', 'heading', 'note', 'list');

-- Table des langages de programmation
CREATE TABLE IF NOT EXISTS folders(
   id_folder UUID PRIMARY KEY,
   folder_name VARCHAR(50) NOT NULL UNIQUE,
   folder_slug VARCHAR(50) NOT NULL UNIQUE,
   parent_id UUID NULL,
   created_at TIMESTAMPTZ NOT NULL  DEFAULT NOW(),
   FOREIGN KEY(parent_id) REFERENCES folders(id_folder) ON DELETE CASCADE
);

-- Table des tags (hooks, performance, memoization, etc.)
CREATE TABLE IF NOT EXISTS tags(
   id_tag UUID PRIMARY KEY,
   tag_name VARCHAR(50) NOT NULL UNIQUE,
   tag_slug VARCHAR(50) NOT NULL UNIQUE,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Table principale des notes/articles
CREATE TABLE IF NOT EXISTS notes(
   id_note UUID PRIMARY KEY,
   note_title VARCHAR(255) NOT NULL,
   note_slug VARCHAR(255) NOT NULL UNIQUE,
   note_subtitle VARCHAR(500),
   note_id_folder UUID NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   FOREIGN KEY(note_id_folder) REFERENCES folders(id_folder)
);

   CREATE INDEX idx_notes_folder ON notes (note_id_folder);
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
   block_type block NOT NULL,
   content TEXT NOT NULL,
   order_index INT NOT NULL,
   metadata JSON,  -- Pour stocker le langage du code, niveau de heading, etc.
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
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

INSERT INTO folders (id_folder, parent_id, folder_name, folder_slug) VALUES
('650e8400-5440-0000-0000-000000000005', '550e8400-5440-0000-0000-000000000005', 'Hooks', 'hooks'),
('650e8400-5440-0000-0000-000000000006', '550e8400-5440-0000-0000-000000000005', 'Performance', 'performance'),
('650e8400-5440-0000-0000-000000000007', '550e8400-5440-0000-0000-000000000001', 'Bases du langage', 'bases-du-langage'),
('650e8400-5440-0000-0000-000000000008', '550e8400-5440-0000-0000-000000000004', 'Ownership & Borrowing', 'ownership-borrowing');

INSERT INTO tags (id_tag, tag_name, tag_slug) VALUES
('750e8400-5440-0000-0000-000000000001', 'hooks', 'hooks'),
('750e8400-5440-0000-0000-000000000002', 'performance', 'performance'),
('750e8400-5440-0000-0000-000000000003', 'memoization', 'memoization');

-- Notes de test
INSERT INTO notes (id_note, title, subtitle, id_folder, note_slug) VALUES
('850e8400-5440-0000-0000-000000000001', 'Comprendre useState', 'Le hook fondamental pour gérer l''état local en React', '650e8400-5440-0000-0000-000000000005', 'comprendre-usestate'),
('850e8400-5440-0000-0000-000000000002', 'L''ownership en Rust', 'Le système de propriété qui garantit la sécurité mémoire sans GC', '650e8400-5440-0000-0000-000000000008', 'l-ownership-en-rust'),
('850e8400-5440-0000-0000-000000000003', 'Les closures en JavaScript', 'Comprendre la portée lexicale et les fermetures', '650e8400-5440-0000-0000-000000000007', 'les-closures-en-javascript');

-- Blocs de la note useState
INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata) VALUES
('950e8400-5440-0000-0000-000000000001', '850e8400-5440-0000-0000-000000000001', 'heading', 'Qu''est-ce que useState ?', 1, NULL),
('950e8400-5440-0000-0000-000000000002', '850e8400-5440-0000-0000-000000000001', 'text', 'useState est un hook React qui permet d''ajouter un état local à un composant fonctionnel. Il retourne un tableau contenant la valeur actuelle et une fonction pour la mettre à jour.', 2, NULL),
('950e8400-5440-0000-0000-000000000003', '850e8400-5440-0000-0000-000000000001', 'code', 'const [count, setCount] = useState(0);

function handleClick() {
  setCount(count + 1);
}', 3, '{"language": "javascript"}'),
('950e8400-5440-0000-0000-000000000004', '850e8400-5440-0000-0000-000000000001', 'note', 'Chaque appel à la fonction setter déclenche un re-render du composant. Ne jamais muter l''état directement.', 4, NULL);

-- Blocs de la note ownership Rust
INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata) VALUES
('950e8400-5440-0000-0000-000000000005', '850e8400-5440-0000-0000-000000000002', 'heading', 'Les trois règles de l''ownership', 1, NULL),
('950e8400-5440-0000-0000-000000000006', '850e8400-5440-0000-0000-000000000002', 'list', '1. Chaque valeur a un propriétaire unique\n2. Il ne peut y avoir qu''un seul propriétaire à la fois\n3. Quand le propriétaire sort de portée, la valeur est libérée', 2, NULL),
('950e8400-5440-0000-0000-000000000007', '850e8400-5440-0000-0000-000000000002', 'code', 'let s1 = String::from("hello");
let s2 = s1; // s1 est déplacé dans s2

// println!("{}", s1); // Erreur ! s1 n''est plus valide
println!("{}", s2); // OK', 3, '{"language": "rust"}'),
('950e8400-5440-0000-0000-000000000008', '850e8400-5440-0000-0000-000000000002', 'note', 'Pour les types qui implémentent Copy (i32, bool, char...), la valeur est copiée automatiquement, pas déplacée.', 4, NULL);

-- Blocs de la note closures JS
INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata) VALUES
('950e8400-5440-0000-0000-000000000009', '850e8400-5440-0000-0000-000000000003', 'heading', 'Qu''est-ce qu''une closure ?', 1, NULL),
('950e8400-5440-0000-0000-000000000010', '850e8400-5440-0000-0000-000000000003', 'text', 'Une closure est une fonction qui "se souvient" de son environnement lexical, même quand elle est exécutée en dehors de cet environnement. Elle capture les variables de la portée parente.', 2, NULL),
('950e8400-5440-0000-0000-000000000011', '850e8400-5440-0000-0000-000000000003', 'code', 'function makeCounter() {
  let count = 0;
  return function() {
    count++;
    return count;
  };
}

const counter = makeCounter();
console.log(counter()); // 1
console.log(counter()); // 2', 3, '{"language": "javascript"}');

-- Tags sur les notes
INSERT INTO note_tags (id_note, id_tag) VALUES
('850e8400-5440-0000-0000-000000000001', '750e8400-5440-0000-0000-000000000001'), -- useState + hooks
('850e8400-5440-0000-0000-000000000001', '750e8400-5440-0000-0000-000000000002'), -- useState + performance
('850e8400-5440-0000-0000-000000000002', '750e8400-5440-0000-0000-000000000002'); -- ownership + performance
