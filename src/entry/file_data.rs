use std::{cmp, collections::HashMap};

use crate::{
    constants,
    entry::parts::{Function, Line},
};

pub enum ItemType {
    File(FileData),
    Folder(FolderData),
}

pub struct FolderData {
    pub name: String,
    pub items: Vec<ItemType>,

    pub function_count: u32,
    pub function_hit_count: u32,

    pub line_count: u32,
    pub line_hit_count: u32,
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
    pub fn new(sf: &str) -> Result<FileData, ()> {
        let mut sf_split: Vec<String> = sf.split(['/', '\\']).map(|s| s.to_string()).collect();

        let name = sf_split.last().ok_or(())?.to_string();

        if name.is_empty() {
            return Err(());
        }

        if sf_split.len() > 1 && sf_split[0].is_empty() {
            sf_split[0] = constants::ROOT_FOLDER_NAME.into();
        }

        let path = &sf_split[..sf_split.len() - 1];

        Ok(FileData {
            name,
            path: path.to_vec(),
            function_count: 0,
            function_hit_count: 0,
            functions: vec![],
            line_count: 0,
            line_hit_count: 0,
            lines: vec![],
        })
    }

    pub fn merge_with(&self, other: &FileData) -> Self {
        let mut lines_map: HashMap<u32, &Line> =
            other.lines.iter().map(|item| (item.number, item)).collect();

        let mut functions_map: HashMap<String, &Function> = other
            .functions
            .iter()
            .map(|item| (item.name.clone(), item))
            .collect();

        let mut lines: Vec<Line> = self
            .lines
            .iter()
            .map(|l| {
                if let Some(matched) = lines_map.remove(&l.number) {
                    l.merge_with(matched)
                } else {
                    l.clone()
                }
            })
            .collect();

        let mut functions: Vec<Function> = self
            .functions
            .iter()
            .map(|l| {
                if let Some(matched) = functions_map.remove(&l.name) {
                    l.merge_with(matched)
                } else {
                    l.clone()
                }
            })
            .collect();

        lines_map.values().for_each(|&v| lines.push(v.clone()));

        functions_map
            .values()
            .for_each(|&v| functions.push(v.clone()));

        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            function_count: cmp::max(self.function_count, other.function_count),
            function_hit_count: cmp::max(self.function_hit_count, other.function_hit_count),
            line_count: cmp::max(self.line_count, other.line_count),
            line_hit_count: cmp::max(self.line_hit_count, other.line_hit_count),
            functions,
            lines,
        }
    }
}

impl FolderData {
    pub fn new(name: &str) -> FolderData {
        FolderData {
            name: name.to_string(),
            items: vec![],
            function_count: 0,
            function_hit_count: 0,
            line_count: 0,
            line_hit_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sf_fails() {
        let sf = "";
        let file_data = FileData::new(sf);

        assert!(file_data.is_err());
    }

    #[test]
    fn simple_sf_passes() {
        let sf = "file.txt";
        let file_data = FileData::new(sf);

        assert!(file_data.is_ok());
    }

    #[test]
    fn deep_sf_passes() {
        let sf = "/home/project/file.txt";
        let file_data = FileData::new(sf);

        assert!(file_data.is_ok());
    }

    #[test]
    fn empty_files_merge() {
        let file_data_1 = FileData::new("/home/project/file.txt").unwrap();
        let file_data_2 = FileData::new("/home/project/file.txt").unwrap();

        let merged = file_data_1.merge_with(&file_data_2);

        assert!(merged.function_count == 0);
        assert!(merged.function_hit_count == 0);
        assert!(merged.line_count == 0);
        assert!(merged.line_hit_count == 0);
        assert!(merged.functions.len() == 0);
        assert!(merged.lines.len() == 0);
    }

    #[test]
    fn populated_files_merge() {
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

        let merged = file_data_1.merge_with(&file_data_2);

        assert!(merged.function_count == 22);
        assert!(merged.function_hit_count == 5);
        assert!(merged.line_count == 344);
        assert!(merged.line_hit_count == 285);
        assert!(merged.functions.len() == 2);
        assert!(merged.lines.len() == 2);

        let first_line = merged.lines.first().unwrap();
        assert!(first_line.number == 12);
        assert!(first_line.execute_count == 13);

        let last_line = merged.lines.last().unwrap();
        assert!(last_line.number == 44);
        assert!(last_line.execute_count == 2);

        let first_function = merged.functions.first().unwrap();
        assert!(first_function.name == "function1");
        assert!(first_function.line_start == 22);
        assert!(first_function.execute_count == 3);

        let last_function = merged.functions.last().unwrap();
        assert!(last_function.name == "function2");
        assert!(last_function.line_start == 29);
        assert!(last_function.execute_count == 11);
    }
}
