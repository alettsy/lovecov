use std::collections::hash_map::Entry;

use crate::{
    constants,
    errors::LovecovError,
    parser::file_data::{FileData, FolderData},
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

    pub fn add_file(&mut self, file: FileData) -> Result<(), LovecovError> {
        let mut current = &mut self.root;

        for part in &file.path {
            current = current
                .folders
                .entry(part.clone())
                .or_insert_with(|| Box::new(FolderData::new(part)))
                .as_mut();
        }

        match current.files.entry(file.name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(file);
            }
            Entry::Occupied(mut entry) => {
                let existing = entry.get_mut();
                let merged = existing.merge_with(&file);
                *existing = merged;
            }
        }

        Ok(())
    }

    pub fn post_process(&mut self) {
        self.root.post_process();
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parts::{Function, Line};

    use super::*;

    #[test]
    fn new_tree_is_empty() {
        let tree = FileTree::new();
        assert!(tree.root.files.is_empty());
        assert!(tree.root.folders.is_empty());
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

        assert!(tree.root.files.len() == 1);
        assert!(tree.root.folders.is_empty());
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

        assert!(tree.root.folders.len() == 1);
        assert!(tree.root.files.is_empty());

        if let Some((_, folder1)) = tree.root.folders.iter().next() {
            assert!(folder1.folders.len() == 1);
            assert!(folder1.files.is_empty());
            assert!(folder1.name == "home");

            if let Some((_, folder2)) = folder1.folders.iter().next() {
                assert!(folder2.files.len() == 1);
                assert!(folder2.folders.is_empty());
                assert!(folder2.name == "project");

                if let Some((_, file1)) = folder2.files.iter().next() {
                    assert!(file1.name == "file1.txt");
                } else {
                    panic!("Missing file")
                }
            } else {
                panic!("Missing project folder")
            }
        } else {
            panic!("Missing home folder")
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

        assert!(tree.root.folders.len() == 1);
        assert!(tree.root.files.is_empty());

        if let Some((_, folder1)) = tree.root.folders.iter().next() {
            assert!(folder1.folders.len() == 1);
            assert!(folder1.files.is_empty());
            assert!(folder1.name == "home");

            if let Some((_, folder2)) = folder1.folders.iter().next() {
                assert!(folder2.files.len() == 1);
                assert!(folder2.folders.is_empty());
                assert!(folder2.name == "project");

                if let Some((_, file1)) = folder2.files.iter().next() {
                    assert!(file1.name == "file1.txt");
                } else {
                    panic!("Missing file")
                }
            } else {
                panic!("Missing project folder")
            }
        } else {
            panic!("Missing home folder")
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

        assert!(tree.root.folders.len() == 1);
        assert!(tree.root.files.is_empty());

        if let Some((_, folder1)) = tree.root.folders.iter().next() {
            assert!(folder1.folders.len() == 1);
            assert!(folder1.files.is_empty());
            assert!(folder1.name == "home");

            if let Some((_, folder2)) = folder1.folders.iter().next() {
                assert!(folder2.files.len() == 2);
                assert!(folder2.folders.is_empty());
                assert!(folder2.name == "project");

                if let Some(file1) = folder2.files.get("file1.txt") {
                    assert!(file1.name == "file1.txt");
                } else {
                    panic!("Missing file")
                }

                if let Some(file1) = folder2.files.get("file2.txt") {
                    assert!(file1.name == "file2.txt");
                } else {
                    panic!("Missing file")
                }
            } else {
                panic!("Missing project folder")
            }
        } else {
            panic!("Missing home folder")
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

        assert!(tree.root.folders.len() == 1);
        assert!(tree.root.files.is_empty());

        if let Some((_, folder1)) = tree.root.folders.iter().next() {
            assert!(folder1.folders.len() == 2);
            assert!(folder1.files.is_empty());
            assert!(folder1.name == "home");

            if let Some(folder2) = folder1.folders.get("project") {
                assert!(folder2.files.len() == 1);
                assert!(folder2.folders.is_empty());
                assert!(folder2.name == "project");

                if let Some((_, file1)) = folder2.files.iter().next() {
                    assert!(file1.name == "file1.txt");
                } else {
                    panic!("Missing file")
                }
            } else {
                panic!("Missing project folder")
            }

            if let Some(folder3) = folder1.folders.get("test") {
                assert!(folder3.files.len() == 1);
                assert!(folder3.folders.is_empty());
                assert!(folder3.name == "test");

                if let Some((_, file1)) = folder3.files.iter().next() {
                    assert!(file1.name == "file1.txt");
                } else {
                    panic!("Missing file")
                }
            } else {
                panic!("Missing test folder")
            }
        } else {
            panic!("Missing home folder")
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

        assert!(tree.root.folders.len() == 1);
        assert!(tree.root.files.is_empty());

        if let Some((_, folder1)) = tree.root.folders.iter().next() {
            assert!(folder1.folders.len() == 2);
            assert!(folder1.files.is_empty());
            assert!(folder1.name == "home");
            assert!(folder1.function_count == 42);
            assert!(folder1.function_hit_count == 16);
            assert!(folder1.line_count == 1019);
            assert!(folder1.line_hit_count == 641);

            if let Some(subfolder1) = folder1.folders.get("test") {
                assert!(subfolder1.files.len() == 1);
                assert!(subfolder1.folders.is_empty());
                assert!(subfolder1.name == "test");
                assert!(subfolder1.function_count == 22);
                assert!(subfolder1.function_hit_count == 2);
                assert!(subfolder1.line_count == 330);
                assert!(subfolder1.line_hit_count == 285);
            } else {
                panic!("Missing test folder")
            }

            if let Some(subfolder2) = folder1.folders.get("project") {
                assert!(subfolder2.files.len() == 2);
                assert!(subfolder2.folders.is_empty());
                assert!(subfolder2.name == "project");
                assert!(subfolder2.function_count == 20);
                assert!(subfolder2.function_hit_count == 14);
                assert!(subfolder2.line_count == 689);
                assert!(subfolder2.line_hit_count == 356);
            } else {
                panic!("Missing project folder")
            }
        } else {
            panic!("Missing home folder")
        }
    }
}
