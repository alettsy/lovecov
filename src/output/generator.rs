use crate::{
    record::file_tree::FileTree,
    errors::LovecovError,
    output::{OutputGenerator, html::HtmlOutput, summary::SummaryOutput},
};
use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum Generator {
    Summary,
    Html,
}

impl Generator {
    pub fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        match self {
            Generator::Summary => SummaryOutput::generate(tree)?,
            Generator::Html => HtmlOutput::generate(tree)?,
        }

        Ok(())
    }
}
