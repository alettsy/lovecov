use crate::{
    entry::file_tree::FileTree, errors::LovecovError, output::{OutputGenerator, html::HtmlOutput, summary::SummaryOutput},
};

pub enum Generator {
    Summary,
    Html,
}

impl Generator {
    pub fn from_str(name: &str) -> Result<Self, LovecovError> {
        match name {
            "summary" => Ok(Generator::Summary),
            "html" => Ok(Generator::Html),
            _ => Err(LovecovError::UsageError),
        }
    }

    pub fn generate(&self, tree: &FileTree) {
        match self {
            Generator::Summary => SummaryOutput::generate(tree),
            Generator::Html => HtmlOutput::generate(tree),
        }
    }
}
