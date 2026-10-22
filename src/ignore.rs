use regex::Regex;

use crate::errors::LovecovError;

pub struct IgnoreCondition {
    regex: Regex,
}

impl IgnoreCondition {
    pub fn from_str(s: &str) -> Result<Self, LovecovError> {
        if s.is_empty() {
            return Err(LovecovError::InvalidIgnoreCondition);
        }

        let split: Vec<_> = s
            .replace("/", "\\/")
            .replace(".", "\\.")
            .replace("*", ".*")
            .split(",")
            .map(|s| format!("({})", s))
            .collect();

        let regex =
            Regex::new(&split.join("|")).map_err(|_| LovecovError::InvalidIgnoreCondition)?;

        Ok(Self { regex })
    }

    pub fn should_ignore(&self, s: &str) -> bool {
        self.regex.is_match(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ignore_args() {
        let cases = vec![""];

        for case in cases {
            let condition = IgnoreCondition::from_str(case);
            assert!(condition.is_err());
        }
    }

    #[test]
    fn valid_ignore_args() {
        let cases = vec![
            "apple",
            "apple,banana",
            "apple,ban*.dart",
            "check,path/to/file.txt",
            "*",
            "*.hide.*",
        ];

        for case in cases {
            let condition = IgnoreCondition::from_str(case);
            assert!(condition.is_ok());
        }
    }

    #[test]
    fn valid_hide_files() {
        let condition = IgnoreCondition::from_str("*.hide.*").unwrap();

        let to_ignore = vec![
            "/file/in/my_file.hide.txt",
            "something.hide.dart",
            ".hide.invis",
            "hide.hide.hide",
            "C:/folder/test.hide.app",
        ];

        let to_show = vec![
            "/file/in/my_file_hide.txt",
            "C:/folder/test_hide.app",
            "hide.txt",
            "/some/other/file.dart",
        ];

        for case in to_ignore {
            assert!(condition.should_ignore(case));
        }

        for case in to_show {
            assert!(!condition.should_ignore(case));
        }
    }

    #[test]
    fn valid_mult_condition() {
        let condition = IgnoreCondition::from_str("txt,*_skip.dart,/hidden_folder/*").unwrap();

        let to_ignore = vec![
            "/file/in/text.txt",
            "/project/hidden_folder/subfolder/test.cs",
            "file.txt",
            "C:/somewhere/hide/some_skip.dart",
            "/project/hidden_folder/subfolder/long_file_skip.dart",
            "txt",
        ];

        let to_show = vec![
            "/file/in/my_file_hide.text",
            "C:/folder/test_hide.app",
            "/somewhere/skip.dart",
            "/some/other/file.dart",
            "lonesome_file.cpp",
        ];

        for case in to_ignore {
            assert!(condition.should_ignore(case));
        }

        for case in to_show {
            assert!(!condition.should_ignore(case));
        }
    }
}
