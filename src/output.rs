use crate::entry::file_tree::FileTree;

pub mod generator;
pub mod html;
pub mod summary;

pub trait OutputGenerator {
    fn generate(tree: &FileTree);
}
