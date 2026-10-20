use std::{io::Write, path::Path, rc::Rc, sync::OnceLock};

use regex::Regex;

use crate::{
    errors::LovecovError,
    io::file_system::{FileSystem, RealFileSystem},
};

#[derive(Clone)]
pub struct FileOutput {
    default_file_name: String,
    output_path: Option<String>,
    file_sys: Rc<dyn FileSystem>,
}

impl FileOutput {
    pub fn from_path_and_default(output_path: Option<String>, default_file_name: String) -> Self {
        FileOutput {
            default_file_name,
            output_path,
            file_sys: Rc::new(RealFileSystem {}),
        }
    }

    pub fn open_write(&self) -> Result<Box<dyn Write>, LovecovError> {
        let output_path = self
            .process_path()
            .map_err(|_| LovecovError::InvalidFilePath)?;

        self.file_sys
            .open_write(Path::new(&output_path))
            .map_err(|_| LovecovError::FileWrite)
    }

    fn process_path(&self) -> Result<String, ()> {
        if self.output_path.is_none() {
            Ok(self.default_file_name.clone())
        } else {
            let provided_path = self.output_path.clone().unwrap();

            if provided_path.is_empty() {
                return Err(());
            }

            if !Self::is_valid_path(&provided_path) {
                return Err(());
            }

            let mut path_parts: Vec<String> = provided_path
                .split(['/', '\\'])
                .map(|s| s.to_string())
                .collect();

            if path_parts.is_empty() {
                return Err(());
            }

            let last_part = path_parts.last().unwrap();
            let is_file = Self::check_is_file(&last_part);

            let len_to_remove = if is_file { 1 } else { 0 };
            let to_create = path_parts[..path_parts.len() - len_to_remove].join("/");

            self.file_sys
                .create_dir_all(Path::new(&to_create))
                .map_err(|_| ())?;

            if is_file {
                Ok(provided_path)
            } else {
                path_parts.push(self.default_file_name.clone());
                let final_path = path_parts.join("/");
                Ok(final_path)
            }
        }
    }

    fn check_is_file(str: &str) -> bool {
        str.contains(".")
    }

    fn is_valid_path(path: &str) -> bool {
        static PATH_REGEX: OnceLock<Regex> = OnceLock::new();

        PATH_REGEX
            .get_or_init(|| {
                Regex::new(
                    r"^(?:[a-zA-Z]:)?(?:[/\\]?[a-zA-Z0-9_\-\.]+(?:[/\\][a-zA-Z0-9_\-\.]+)*)?$",
                )
                .unwrap()
            })
            .is_match(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockFileSystem;

    impl FileSystem for MockFileSystem {
        fn create_dir_all(&self, _path: &Path) -> std::io::Result<()> {
            Ok(())
        }

        fn open_write(&self, _path: &Path) -> std::io::Result<Box<dyn Write>> {
            Ok(Box::new(vec![]))
        }
    }

    #[test]
    fn is_file_valid() {
        let cases = vec![
            "file.txt",
            "file.tar.gz",
            "super_long-namedFile.zip",
            ".invis-file",
        ];

        for case in cases {
            assert!(FileOutput::check_is_file(case));
        }
    }

    #[test]
    fn is_file_invalid() {
        let cases = vec!["", "file", "very_long-file"];

        for case in cases {
            assert!(!FileOutput::check_is_file(case));
        }
    }

    #[test]
    fn process_path_valid() {
        let cases: Vec<String> = vec![
            "local_folder".into(),
            "local_file.txt".into(),
            "/full/path/to/folder".into(),
            "relative/path/to/folder".into(),
            "C:/full/path/to/folder".into(),
            "/full/path/to/file.txt".into(),
            "relative/path/to/file.txt".into(),
            "C:/full/path/to/file.txt".into(),
        ];

        for case in cases {
            let case_copy = case.clone();
            let file_out = FileOutput {
                output_path: Some(case),
                default_file_name: "default.txt".into(),
                file_sys: Rc::new(MockFileSystem {}),
            };
            let result = file_out.process_path();

            assert!(result.is_ok(), "failed at case {}", case_copy);
        }
    }

    #[test]
    fn process_path_invalid() {
        let cases: Vec<String> = vec![
            "".into(),
            "%".into(),
            "*&!@".into(),
            "///full/path/to/folder".into(),
            "/full/path/to////folder".into(),
            "relat^&#ive/path/to/folder".into(),
            "AAA:/full/path/to/folder".into(),
            "C::/full/path/to/folder".into(),
            "C:/full/path//to/folder".into(),
            "/full/path/C:/file.txt".into(),
        ];

        for case in cases {
            let case_copy = case.clone();
            let file_out = FileOutput {
                output_path: Some(case),
                default_file_name: "default.txt".into(),
                file_sys: Rc::new(MockFileSystem {}),
            };
            let result = file_out.process_path();

            assert!(result.is_err(), "failed at case {}", case_copy);
        }
    }
}
