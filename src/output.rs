use crate::{errors::LovecovError, record::file_tree::FileTree};

pub mod generator;

mod file_output;
mod file_system;
mod html;
mod summary;

pub trait OutputGenerator {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError>;
}
