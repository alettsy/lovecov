use crate::{errors::LovecovError, parser::file_tree::FileTree};

pub mod generator;

mod csv;
mod html;
mod summary;

pub trait OutputGenerator {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError>;
}
