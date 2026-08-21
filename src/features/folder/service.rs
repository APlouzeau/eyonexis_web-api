use std::collections::HashMap;

use uuid::Uuid;

use super::model::FolderNode;
use super::repository::FolderRepository;
use crate::features::folder::model::FolderBranch;
use crate::features::note::model::NoteToList;
use crate::features::note::service::NoteService;

#[derive(Clone)]
pub struct FolderService<R: FolderRepository> {
    pub repository: R,
    pub note_service: NoteService<R, R: NoteRepository>,
}

impl<R: FolderRepository> FolderService<R> {
    pub async fn get_folder_tree(&self) -> Result<Vec<FolderNode>, sqlx::Error> {
        let folders = self.repository.get_folder_tree().await?;
        let mut child_per_parent: HashMap<Option<Uuid>, Vec<FolderBranch>> = HashMap::new();
        let mut result: Vec<FolderNode> = Vec::new();

        for folder in &folders {
            child_per_parent
                .entry(folder.parent_id)
                .or_insert_with(Vec::new)
                .push(folder.clone())
        }

        if let Some(roots) = child_per_parent.get(&None) {
            for root in roots {
                result.push(build_node(root.clone(), &child_per_parent));
            }
        }

        Ok(result)
    }

    /*     pub async fn create(&self, new_folder: NewFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.create(new_folder).await?;
        self.get_all().await
    }

    pub async fn delete(&self, id: DeleteFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}

fn build_node(
    folder: FolderBranch,
    child_per_parent: &HashMap<Option<Uuid>, Vec<FolderBranch>>,
) -> FolderNode {
    let childs = child_per_parent.get(&Some(folder.id_folder));
    let mut children: Vec<FolderNode> = Vec::new();

    if let Some(childs) = childs {
        for child in childs {
            children.push(build_node(child.clone(), child_per_parent))
        }
    }

    let notes: Vec<NoteToList> = Vec::new();

    FolderNode {
        id_folder: folder.id_folder,
        folder_name: folder.folder_name,
        children,
        notes,
    }
}

fn attach_notes(node: FolderNode, notes_per_folder: &HashMap<Uuid, Vec<NoteToList>>) -> FolderNode {
    let notes = notes_per_folder
        .get(&node.id_folder)
        .cloned()
        .unwrap_or_default();

    let mut children = Vec::new();
    for child in node.children {
        children.push(attach_notes(child, notes_per_folder));
    }

    FolderNode {
        id_folder: node.id_folder,
        folder_name: node.folder_name,
        children,
        notes,
    }
}
