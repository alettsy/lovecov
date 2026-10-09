use std::cmp;

#[derive(Clone)]
pub struct Line {
    pub number: u32,
    pub execute_count: u32,
}

impl Line {
    pub fn new(number: u32, execute_count: u32) -> Self {
        Self {
            number,
            execute_count,
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub line_start: u32,
    pub execute_count: u32,
}

impl Function {
    pub fn new(name: &str, line_start: u32) -> Self {
        Self {
            name: name.to_string(),
            line_start,
            execute_count: 0,
        }
    }
}

pub enum ItemType {
    File(FileData),
    Folder(FolderData),
}

pub struct FolderData {
    pub name: String,
    pub items: Vec<ItemType>,
}

#[derive(Clone)]
pub struct FileData {
    pub name: String,
    pub path: Vec<String>,

    pub function_count: u32,
    pub function_hit_count: u32,
    pub functions: Vec<Function>,

    pub line_count: u32,
    pub line_hit_count: u32,
    pub lines: Vec<Line>,
}

impl FileData {
    pub fn new(sf: &str) -> FileData {
        let sf_split: Vec<String> = sf
            .split(|c| c == '/' || c == '\\')
            .map(|s| s.to_string())
            .collect();
        let name = sf_split.last().expect("Invalid SF file name").to_string();
        let path = &sf_split[..sf_split.len() - 1];

        FileData {
            name,
            path: path.to_vec(),
            function_count: 0,
            function_hit_count: 0,
            functions: vec![],
            line_count: 0,
            line_hit_count: 0,
            lines: vec![],
        }
    }

    pub fn merge_with(&self, other: &FileData) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            function_count: cmp::max(self.function_count, other.function_count),
            function_hit_count: cmp::max(self.function_hit_count, other.function_hit_count),
            line_count: cmp::max(self.line_count, other.line_count),
            line_hit_count: cmp::max(self.line_hit_count, other.line_hit_count),
            functions: vec![],
            lines: vec![],
        }
    }
}

impl FolderData {
    pub fn new(name: &str) -> FolderData {
        FolderData {
            name: name.to_string(),
            items: vec![],
        }
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }
}
