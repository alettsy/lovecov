use std::fs;

use crate::{
    entry::{
        data::{FolderData, ItemType},
        file_tree::FileTree,
    },
    output::OutputGenerator,
};

pub struct HtmlOutput;

impl OutputGenerator for HtmlOutput {
    fn generate(tree: &FileTree) {
        let mut lines: Vec<String> = vec![];
        Self::generate_html(&tree.root, &mut lines);
        fs::write("output.html", lines.join("\n")).unwrap();
    }
}

impl HtmlOutput {
    fn generate_html(folder: &FolderData, lines: &mut Vec<String>) {
        for item in &folder.items {
            match item {
                ItemType::File(file) => {
                    lines.push(String::from(format!("<div>{}</div>", file.name)));
                }
                ItemType::Folder(folder) => {
                    lines.push(String::from("<details>"));
                    lines.push(String::from(format!("<summary>{}</summary", folder.name)));
                    Self::generate_html(folder, lines);
                    lines.push(String::from("</details>"));
                }
            }
        }
    }
}
