use crate::{
    constants,
    record::file_data::{FileData, FolderData, ItemType},
    errors::LovecovError,
};

pub struct FileTree {
    pub root: FolderData,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: FolderData::new(constants::ROOT_FOLDER_NAME),
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
                    ItemType::Folder(_) => Err(LovecovError::Tree(
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
            ItemType::File(_) => Err(LovecovError::Tree(
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

#[cfg(test)]
mod tests {
    use crate::record::parts::{Function, Line};

    use super::*;

    #[test]
    fn new_tree_is_empty() {
        let tree = FileTree::new();
        assert!(tree.root.items.len() == 0);
    }

    #[test]
    fn add_shallow_file() {
        let mut tree = FileTree::new();

        let file_data = FileData {
            name: "file1.txt".into(),
            path: vec![],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        tree.add_file(file_data).unwrap();

        assert!(tree.root.items.len() == 1);

        let first_record = tree.root.items.first().unwrap();

        assert!(matches!(first_record, ItemType::File(_)));
    }

    #[test]
    fn add_deep_file() {
        let mut tree = FileTree::new();

        let file_data = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        tree.add_file(file_data).unwrap();

        assert!(tree.root.items.len() == 1);

        if let Some(ItemType::Folder(a)) = tree.root.items.first() {
            assert!(a.name == "home");
            assert!(a.items.len() == 1);
            if let Some(ItemType::Folder(b)) = a.items.first() {
                assert!(b.name == "project");
                assert!(b.items.len() == 1);

                let file_record = b.items.first().unwrap();
                assert!(matches!(file_record, ItemType::File(_)));
            } else {
                panic!("Expected project folder");
            }
        } else {
            panic!("Expected home folder");
        }
    }

    #[test]
    fn add_duplicate_file() {
        let mut tree = FileTree::new();

        let file_data_1 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        let file_data_2 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 22,
            function_hit_count: 2,
            line_count: 330,
            line_hit_count: 285,
            functions: vec![Function {
                name: "function2".into(),
                line_start: 29,
                execute_count: 10,
            }],
            lines: vec![Line::new(12, 8)],
        };

        tree.add_file(file_data_1).unwrap();
        tree.add_file(file_data_2).unwrap();

        assert!(tree.root.items.len() == 1);

        if let Some(ItemType::Folder(a)) = tree.root.items.first() {
            assert!(a.name == "home");
            assert!(a.items.len() == 1);

            if let Some(ItemType::Folder(b)) = a.items.first() {
                assert!(b.name == "project");
                assert!(b.items.len() == 1);

                let file_record = b.items.first().unwrap();
                assert!(matches!(file_record, ItemType::File(_)));
            } else {
                panic!("Expected project folder");
            }
        } else {
            panic!("Expected home folder");
        }
    }

    #[test]
    fn separate_file_creates_own_record() {
        let mut tree = FileTree::new();

        let file_data_1 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        let file_data_2 = FileData {
            name: "file2.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 22,
            function_hit_count: 2,
            line_count: 330,
            line_hit_count: 285,
            functions: vec![Function {
                name: "function2".into(),
                line_start: 29,
                execute_count: 10,
            }],
            lines: vec![Line::new(12, 8)],
        };

        tree.add_file(file_data_1).unwrap();
        tree.add_file(file_data_2).unwrap();

        assert!(tree.root.items.len() == 1);

        if let Some(ItemType::Folder(a)) = tree.root.items.first() {
            assert!(a.name == "home");
            assert!(a.items.len() == 1);

            if let Some(ItemType::Folder(b)) = a.items.first() {
                assert!(b.name == "project");
                assert!(b.items.len() == 2);

                let file_record_1 = b.items.first().unwrap();
                let file_record_2 = b.items.last().unwrap();
                assert!(matches!(file_record_1, ItemType::File(_)));
                assert!(matches!(file_record_2, ItemType::File(_)));
            } else {
                panic!("Expected project folder");
            }
        } else {
            panic!("Expected home folder");
        }
    }

    #[test]
    fn separate_directory_created() {
        let mut tree = FileTree::new();

        let file_data_1 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        let file_data_2 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "test".into()],
            function_count: 22,
            function_hit_count: 2,
            line_count: 330,
            line_hit_count: 285,
            functions: vec![Function {
                name: "function2".into(),
                line_start: 29,
                execute_count: 10,
            }],
            lines: vec![Line::new(12, 8)],
        };

        tree.add_file(file_data_1).unwrap();
        tree.add_file(file_data_2).unwrap();

        assert!(tree.root.items.len() == 1);

        if let Some(ItemType::Folder(a)) = tree.root.items.first() {
            assert!(a.name == "home");
            assert!(a.items.len() == 2);

            if let Some(ItemType::Folder(b)) = a.items.first() {
                assert!(b.name == "project");
                assert!(b.items.len() == 1);

                let file_record = b.items.first().unwrap();
                assert!(matches!(file_record, ItemType::File(_)));
            } else {
                panic!("Expected project folder");
            }

            if let Some(ItemType::Folder(b)) = a.items.last() {
                assert!(b.name == "test");
                assert!(b.items.len() == 1);

                let file_record = b.items.first().unwrap();
                assert!(matches!(file_record, ItemType::File(_)));
            } else {
                panic!("Expected test folder");
            }
        } else {
            panic!("Expected home folder");
        }
    }

    #[test]
    fn post_process_updates_folders() {
        let mut tree = FileTree::new();

        let file_data_1 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 5,
            line_count: 344,
            line_hit_count: 245,
            functions: vec![
                Function {
                    name: "function1".into(),
                    line_start: 22,
                    execute_count: 3,
                },
                Function {
                    name: "function2".into(),
                    line_start: 29,
                    execute_count: 1,
                },
            ],
            lines: vec![Line::new(12, 5), Line::new(44, 2)],
        };

        let file_data_2 = FileData {
            name: "file1.txt".into(),
            path: vec!["home".into(), "test".into()],
            function_count: 22,
            function_hit_count: 2,
            line_count: 330,
            line_hit_count: 285,
            functions: vec![Function {
                name: "function2".into(),
                line_start: 29,
                execute_count: 10,
            }],
            lines: vec![Line::new(12, 8)],
        };

        let file_data_3 = FileData {
            name: "file3.txt".into(),
            path: vec!["home".into(), "project".into()],
            function_count: 10,
            function_hit_count: 9,
            line_count: 345,
            line_hit_count: 111,
            functions: vec![],
            lines: vec![],
        };

        tree.add_file(file_data_1).unwrap();
        tree.add_file(file_data_2).unwrap();
        tree.add_file(file_data_3).unwrap();

        tree.post_process();

        assert!(tree.root.items.len() == 1);

        if let Some(ItemType::Folder(a)) = tree.root.items.first() {
            assert!(a.name == "home");
            assert!(a.items.len() == 2);
            assert!(a.function_count == 42);
            assert!(a.function_hit_count == 16);
            assert!(a.line_count == 1019);
            assert!(a.line_hit_count == 641);

            if let Some(ItemType::Folder(b)) = a.items.first() {
                assert!(b.name == "project");
                assert!(b.function_count == 20);
                assert!(b.function_hit_count == 14);
                assert!(b.line_count == 689);
                assert!(b.line_hit_count == 356);
            } else {
                panic!("Expected project folder");
            }

            if let Some(ItemType::Folder(b)) = a.items.last() {
                assert!(b.name == "test");
                assert!(b.function_count == 22);
                assert!(b.function_hit_count == 2);
                assert!(b.line_count == 330);
                assert!(b.line_hit_count == 285);
            } else {
                panic!("Expected test folder");
            }
        } else {
            panic!("Expected home folder");
        }
    }
}
