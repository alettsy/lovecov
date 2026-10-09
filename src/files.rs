use std::{cmp, collections::HashMap};

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
                    l.merge_with(&matched)
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
                    l.merge_with(&matched)
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
        }
    }
}

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

    fn merge_with(&self, other: &Line) -> Self {
        Self {
            number: self.number,
            execute_count: self.execute_count + other.execute_count,
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

    fn merge_with(&self, other: &Function) -> Self {
        Self {
            name: self.name.clone(),
            line_start: self.line_start,
            execute_count: self.execute_count + other.execute_count,
        }
    }
}
