use crate::{
    entry::file_tree::FileTree,
    output::{OutputGenerator, html::HtmlOutput, summary::SummaryOutput},
};

pub enum Generator {
    Summary,
    Html,
}

impl Generator {
    pub fn from_str(name: &str) -> Self {
        match name {
            "summary" => Generator::Summary,
            "html" => Generator::Html,
            _ => panic!("Generator {} not recognised", name),
        }
    }

    pub fn generate(&self, tree: &FileTree) {
        match self {
            Generator::Summary => SummaryOutput::generate(tree),
            Generator::Html => HtmlOutput::generate(tree),
        }
    }
}
