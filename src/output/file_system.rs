use std::{fs, path::Path};

pub trait FileSystem {
    fn write(&self, path: &Path, contents: &[u8]) -> std::io::Result<()>;
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> std::io::Result<()> {
        fs::write(path, contents)
    }
}
