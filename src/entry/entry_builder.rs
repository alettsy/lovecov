use crate::{
    constants,
    entry::{
        data::FileData,
        parts::{Function, Line},
    },
    errors::LovecovError,
};

pub struct EntryBuilder {
    pub entry: Option<FileData>,
    reached_end: bool,
}

impl EntryBuilder {
    pub fn new() -> Self {
        Self {
            entry: None,
            reached_end: false,
        }
    }

    pub fn process_tokens(
        &mut self,
        tokens: Vec<&str>,
        line_number: usize,
    ) -> Result<(), LovecovError> {
        let first_token = tokens
            .first()
            .ok_or(LovecovError::MissingToken(line_number))?
            .to_lowercase();

        if self.entry.is_none() {
            if first_token.eq(constants::SOURCE_FILE) {
                if tokens.len() != 2 {
                    return Err(LovecovError::TokenFormatError(
                        line_number,
                        "Invalid SF".into(),
                    ));
                }

                let file_path = tokens[1];
                let file_data = FileData::new(file_path, line_number)?;
                self.entry = Some(file_data);
                return Ok(());
            } else {
                return Err(LovecovError::TokenFormatError(
                    line_number,
                    format!("Invalid initial entry token '{}'", first_token),
                ));
            }
        }

        match first_token.as_str() {
            constants::TEST_NAME => {
                // NOTE: ignoring TN because it is not that important
                Ok(())
            }
            constants::SOURCE_FILE => Err(LovecovError::TokenFormatError(
                line_number,
                "Repeated SF in entry".into(),
            )),
            constants::FUNC_NAME => {
                self.process_function_name(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            constants::FUN_EXEC_COUNT => {
                self.process_function_execute(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            constants::FUNC_COUNT => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid FNF".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.function_count += count;
                }

                Ok(())
            }
            constants::FUNC_HIT_COUNT => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid FNH".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.function_hit_count += count;
                }

                Ok(())
            }
            constants::LINE_EXEC_COUNT => {
                self.process_line_execute_count(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            constants::LINE_COUNT => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid LF".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.line_count += count;
                }

                Ok(())
            }
            constants::LINE_HIT_COUNT => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid LH".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.line_hit_count += count;
                }

                Ok(())
            }
            constants::END_OF_RECORD => {
                self.reached_end = true;
                Ok(())
            }
            _ => Err(LovecovError::UnknownToken(
                line_number,
                String::from(first_token),
            )),
        }
    }

    fn process_line_execute_count(&mut self, tokens: Vec<&str>) -> Result<(), String> {
        let parts = Self::extract_two_parts(tokens).map_err(|_| String::from("Invalid DA"))?;

        let line_number: u32 = parts
            .first()
            .ok_or(String::from("DA missing line number"))?
            .parse()
            .map_err(|_| String::from("DA line number invalid"))?;

        let hit_count_str = parts.last().ok_or(String::from("DA missing hit count"))?;

        let hit_count: u32 = if hit_count_str.eq(&"-") {
            1
        } else {
            hit_count_str
                .parse()
                .map_err(|_| String::from("Hit count invalid in DA"))?
        };

        if let Some(entry) = &mut self.entry {
            entry.lines.push(Line::new(line_number, hit_count));
        }

        Ok(())
    }

    fn process_function_name(&mut self, tokens: Vec<&str>) -> Result<(), String> {
        let parts = Self::extract_two_parts(tokens).map_err(|_| String::from("Invalid FN"))?;

        let line_number: u32 = parts
            .first()
            .ok_or(String::from("FN missing line number"))?
            .parse()
            .map_err(|_| String::from("FN line number invalid"))?;

        let function_name = parts
            .last()
            .ok_or(String::from("FN missing function name"))?;

        if function_name.is_empty() {
            return Err(String::from("Invalid FN"));
        }

        if let Some(entry) = &mut self.entry {
            entry
                .functions
                .push(Function::new(function_name, line_number));
        }

        Ok(())
    }

    fn process_function_execute(&mut self, tokens: Vec<&str>) -> Result<(), String> {
        let parts = Self::extract_two_parts(tokens).map_err(|_| String::from("Invalid FNDA"))?;

        let count: u32 = parts
            .first()
            .ok_or(String::from("FNDA missing count"))?
            .parse()
            .map_err(|_| String::from("FNDA count invalid"))?;

        let function_name = parts
            .last()
            .ok_or(String::from("FNDA missing function name"))?;

        if function_name.is_empty() {
            return Err(String::from("Invalid FN"));
        }

        if let Some(entry) = &mut self.entry {
            let index_opt = entry
                .functions
                .iter()
                .position(|f| f.name.eq(function_name));

            if index_opt.is_none() {
                eprintln!("FNDA function {} not found -- ignoring", function_name);
                return Ok(());
            }

            let index = index_opt.unwrap();
            entry.functions[index].execute_count += count;
        }

        Ok(())
    }

    fn extract_two_parts(tokens: Vec<&str>) -> Result<Vec<&str>, ()> {
        if tokens.len() != 2 {
            return Err(());
        }

        let parts: Vec<&str> = tokens.last().unwrap().split(",").collect();

        if parts.len() != 2 {
            return Err(());
        }

        Ok(parts)
    }

    fn extract_count(tokens: Vec<&str>) -> Result<u32, ()> {
        if tokens.len() != 2 {
            return Err(());
        }

        tokens.last().ok_or(())?.parse::<u32>().map_err(|_| ())
    }

    pub fn is_done(&self) -> bool {
        self.reached_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_token_while_none_fails() {
        let mut entry = EntryBuilder::new();
        let tokens = vec!["DA", "1,2"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_err());
    }

    #[test]
    fn sf_starts_entry() {
        let mut entry = EntryBuilder::new();
        let tokens = vec!["SF", "/home/project/file.txt"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
        assert!(entry.entry.is_some());
    }

    #[test]
    fn sf_invalid() {
        let cases = vec![vec!["SF"], vec!["SF", ""]];

        let mut entry = EntryBuilder::new();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn fn_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["FN", "1,function1"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn fn_invalid_cases() {
        let cases = vec![
            vec!["FN"],
            vec!["FN", ""],
            vec!["FN", "1,"],
            vec!["FN", ",function1"],
            vec!["FN", "A,function1"],
        ];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnda_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["FNDA", "1,function1"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn fnda_invalid_cases() {
        let cases = vec![
            vec!["FNDA"],
            vec!["FNDA", ""],
            vec!["FNDA", "1,"],
            vec!["FNDA", ",function1"],
            vec!["FNDA", "A,function1"],
        ];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["FNF", "34"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn fnf_invalid_cases() {
        let cases = vec![vec!["FNF"], vec!["FNF", ""], vec!["FNF", "A"]];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["FNH", "34"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn fnh_invalid_cases() {
        let cases = vec![vec!["FNH"], vec!["FNH", ""], vec!["FNH", "A"]];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn da_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["DA", "3,4"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn da_invalid_cases() {
        let cases = vec![
            vec!["DA"],
            vec!["DA", ""],
            vec!["DA", "A,3"],
            vec!["DA", "3,A"],
        ];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn lf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["LF", "34"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn lf_invalid_cases() {
        let cases = vec![vec!["LF"], vec!["LF", ""], vec!["LF", "A"]];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn lh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["LH", "34"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn lh_invalid_cases() {
        let cases = vec![vec!["LH"], vec!["LH", ""], vec!["LH", "A"]];

        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        for case in cases {
            let result = entry.process_tokens(case, 4);
            assert!(result.is_err());
        }
    }

    #[test]
    fn end_of_record() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["end_of_record"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_ok());
        assert!(entry.is_done());
    }

    #[test]
    fn invalid_token_fails() {
        let mut entry = EntryBuilder::new();
        let sf_tokens = vec!["SF", "file.txt"];
        entry.process_tokens(sf_tokens, 0).unwrap();

        let tokens = vec!["ZAF"];

        let result = entry.process_tokens(tokens, 4);
        assert!(result.is_err());
    }
}
