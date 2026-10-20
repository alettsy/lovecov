use std::{fs, io::Write, path::Path};

pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()>;
    fn open_write(&self, path: &Path) -> std::io::Result<Box<dyn Write>>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn open_write(&self, path: &Path) -> std::io::Result<Box<dyn Write>> {
        Ok(Box::new(fs::File::create(path)?))
    }
}
