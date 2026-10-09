use crate::files::{FileData, FolderData, ItemType};

pub struct FileTree {
    pub root: FolderData,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: FolderData::new("root"),
        }
    }

    pub fn add_file(&mut self, sf: FileData) {
        Self::add_file_recursive(&mut self.root.items, sf.path.clone(), sf);
    }

    fn add_file_recursive(items: &mut Vec<ItemType>, mut path: Vec<String>, file: FileData) {
        if path.is_empty() {
            let file_index = items.iter().position(|f| match f {
                ItemType::File(f) => file.name == f.name,
                ItemType::Folder(_) => false,
            });

            if let Some(index) = file_index {
                match &mut items[index] {
                    ItemType::File(found_file) => {
                        let merged_file = file.merge_with(found_file);
                        items[index] = ItemType::File(merged_file);
                    }
                    ItemType::Folder(_) => panic!("Matched file points to folder"),
                }
            } else {
                items.push(ItemType::File(file));
            }

            return;
        }

        let name = path.remove(0);

        let folder_index = items.iter().position(|f| match f {
            ItemType::File(_) => false,
            ItemType::Folder(folder) => folder.name == name,
        });

        if folder_index.is_none() {
            items.push(ItemType::Folder(FolderData::new(&name)));
        }

        let index = items
            .iter()
            .position(|f| match f {
                ItemType::File(_) => false,
                ItemType::Folder(folder) => folder.name == name,
            })
            .unwrap();

        match &mut items[index] {
            ItemType::Folder(folder) => {
                Self::add_file_recursive(&mut folder.items, path, file);
            }
            ItemType::File(_) => panic!("Path matched with file, not folder. Bad SF"),
        }
    }
}
