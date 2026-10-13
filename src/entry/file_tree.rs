use crate::{
    entry::data::{FileData, FolderData, ItemType},
    errors::LovecovError,
};

pub struct FileTree {
    pub root: FolderData,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: FolderData::new("root"),
        }
    }

    pub fn add_file(&mut self, sf: FileData) -> Result<(), LovecovError> {
        Self::add_file_recursive(&mut self.root.items, sf.path.clone(), sf)?;
        Ok(())
    }

    fn add_file_recursive(
        items: &mut Vec<ItemType>,
        mut path: Vec<String>,
        file: FileData,
    ) -> Result<(), LovecovError> {
        if path.is_empty() {
            let file_index = items.iter().position(|f| match f {
                ItemType::File(f) => file.name == f.name,
                ItemType::Folder(_) => false,
            });

            return if let Some(index) = file_index {
                match &mut items[index] {
                    ItemType::File(found_file) => {
                        let merged_file = file.merge_with(found_file);
                        items[index] = ItemType::File(merged_file);
                        Ok(())
                    }
                    ItemType::Folder(_) => Err(LovecovError::TreeError(
                        "Expected file, but points to folder".into(),
                    )),
                }
            } else {
                items.push(ItemType::File(file));
                Ok(())
            };
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
                Self::add_file_recursive(&mut folder.items, path, file)?;
                Ok(())
            }
            ItemType::File(_) => Err(LovecovError::TreeError(
                "Path matched with file, not folder. Bad SF".into(),
            )),
        }
    }

    pub fn post_process(&mut self) {
        Self::post_process_recursive(&mut self.root);
    }

    fn post_process_recursive(parent: &mut FolderData) {
        parent.items.sort_by(|a, b| match (a, b) {
            (ItemType::Folder(a_data), ItemType::Folder(b_data)) => a_data.name.cmp(&b_data.name),
            (ItemType::File(a_data), ItemType::File(b_data)) => a_data.name.cmp(&b_data.name),
            (ItemType::Folder(_), ItemType::File(_)) => std::cmp::Ordering::Less,
            (ItemType::File(_), ItemType::Folder(_)) => std::cmp::Ordering::Greater,
        });

        for item in &mut parent.items {
            match item {
                ItemType::File(file) => {
                    parent.function_count += file.function_count;
                    parent.function_hit_count += file.function_hit_count;
                    parent.line_count += file.line_count;
                    parent.line_hit_count += file.line_hit_count;
                }
                ItemType::Folder(folder) => {
                    Self::post_process_recursive(folder);
                    parent.function_count += folder.function_count;
                    parent.function_hit_count += folder.function_hit_count;
                    parent.line_count += folder.line_count;
                    parent.line_hit_count += folder.line_hit_count;
                }
            }
        }
    }
}
