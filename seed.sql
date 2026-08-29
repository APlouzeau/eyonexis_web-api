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
INSERT INTO notes (id_note, title, subtitle, id_folder, slug) VALUES
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
('950e8400-5440-0000-0000-000000000004', '850e8400-5440-0000-0000-000000000001', 'text', 'Chaque appel à la fonction setter déclenche un re-render du composant. Ne jamais muter l''état directement.', 4, NULL);

-- Blocs de la note ownership Rust
INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata) VALUES
('950e8400-5440-0000-0000-000000000005', '850e8400-5440-0000-0000-000000000002', 'heading', 'Les trois règles de l''ownership', 1, NULL),
('950e8400-5440-0000-0000-000000000006', '850e8400-5440-0000-0000-000000000002', 'text', '- Chaque valeur a un propriétaire unique
- Il ne peut y avoir qu''un seul propriétaire à la fois
- Quand le propriétaire sort de portée, la valeur est libérée', 2, NULL),
('950e8400-5440-0000-0000-000000000007', '850e8400-5440-0000-0000-000000000002', 'code', 'let s1 = String::from("hello");
let s2 = s1; // s1 est déplacé dans s2

// println!("{}", s1); // Erreur ! s1 n''est plus valide
println!("{}", s2); // OK', 3, '{"language": "rust"}'),
('950e8400-5440-0000-0000-000000000012', '850e8400-5440-0000-0000-000000000002', 'heading', 'Copy', 4, NULL),
('950e8400-5440-0000-0000-000000000008', '850e8400-5440-0000-0000-000000000002', 'text', 'Pour les types qui implémentent Copy (i32, bool, char...), la valeur est copiée automatiquement, pas déplacée.', 5, NULL);

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
