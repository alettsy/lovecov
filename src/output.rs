use crate::{entry::file_tree::FileTree, errors::LovecovError};

pub mod generator;
pub mod html;
pub mod summary;

pub trait OutputGenerator {
    fn generate(tree: &FileTree) -> Result<(), LovecovError>;
}
