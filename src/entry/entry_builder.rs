use crate::{
    constants,
    entry::{
        data::FileData,
        parts::{Function, Line},
        token::Token,
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
        token: Token,
        data: Option<&String>,
    ) -> Result<(), LovecovError> {
        let data = if data.is_none() {
            vec![]
        } else {
            data.unwrap().split(constants::DATA_SPLIT).collect()
        };

        if self.entry.is_none() {
            if token == Token::SourceFile {
                if data.len() != 1 {
                    return Err(Token::SourceFile.into_err());
                }

                let file_path = data[0];
                let file_data = FileData::new(file_path)?;
                self.entry = Some(file_data);
                return Ok(());
            } else {
                return Err(LovecovError::InvalidStartToken);
            }
        }

        match token {
            Token::TestName => {
                // NOTE: ignoring TN because it is not that important
                Ok(())
            }
            Token::SourceFile => Err(LovecovError::RepeatedSourceTokenError),
            Token::FunctionName => {
                self.process_function_name(data)
                    .map_err(|_| Token::FunctionName.into_err())?;
                Ok(())
            }
            Token::FunctionExecCount => {
                self.process_function_execute(data)
                    .map_err(|_| Token::FunctionExecCount.into_err())?;
                Ok(())
            }
            Token::FunctionCount => {
                let count =
                    Self::extract_count(data).map_err(|_| Token::FunctionCount.into_err())?;

                if let Some(entry) = &mut self.entry {
                    entry.function_count += count;
                }

                Ok(())
            }
            Token::FunctionHitCount => {
                let count =
                    Self::extract_count(data).map_err(|_| Token::FunctionHitCount.into_err())?;

                if let Some(entry) = &mut self.entry {
                    entry.function_hit_count += count;
                }

                Ok(())
            }
            Token::LineExecCount => {
                self.process_line_execute_count(data)
                    .map_err(|_| Token::LineExecCount.into_err())?;
                Ok(())
            }
            Token::LineCount => {
                let count = Self::extract_count(data).map_err(|_| Token::LineCount.into_err())?;

                if let Some(entry) = &mut self.entry {
                    entry.line_count += count;
                }

                Ok(())
            }
            Token::LineHitCount => {
                let count =
                    Self::extract_count(data).map_err(|_| Token::LineHitCount.into_err())?;

                if let Some(entry) = &mut self.entry {
                    entry.line_hit_count += count;
                }

                Ok(())
            }
            Token::EndOfRecord => {
                self.reached_end = true;
                Ok(())
            }
        }
    }

    fn process_line_execute_count(&mut self, tokens: Vec<&str>) -> Result<(), ()> {
        let parts = Self::extract_two_parts(tokens)?;

        let line_number: u32 = parts.first().ok_or(())?.parse().map_err(|_| ())?;

        let hit_count_str = parts.last().ok_or(())?;

        let hit_count: u32 = if hit_count_str.eq(&constants::HIT_EMPTY) {
            1
        } else {
            hit_count_str.parse().map_err(|_| ())?
        };

        if let Some(entry) = &mut self.entry {
            entry.lines.push(Line::new(line_number, hit_count));
        }

        Ok(())
    }

    fn process_function_name(&mut self, tokens: Vec<&str>) -> Result<(), ()> {
        let parts = Self::extract_two_parts(tokens).map_err(|_| ())?;

        let line_number: u32 = parts.first().ok_or(())?.parse().map_err(|_| ())?;

        let function_name = parts.last().ok_or(())?;

        if function_name.is_empty() {
            return Err(());
        }

        if let Some(entry) = &mut self.entry {
            entry
                .functions
                .push(Function::new(function_name, line_number));
        }

        Ok(())
    }

    fn process_function_execute(&mut self, tokens: Vec<&str>) -> Result<(), ()> {
        let parts = Self::extract_two_parts(tokens).map_err(|_| ())?;

        let count: u32 = parts.first().ok_or(())?.parse().map_err(|_| ())?;

        let function_name = parts.last().ok_or(())?;

        if function_name.is_empty() {
            return Err(());
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

    fn extract_two_parts(data: Vec<&str>) -> Result<Vec<&str>, ()> {
        if data.len() != 2 {
            return Err(());
        }

        Ok(data)
    }

    fn extract_count(tokens: Vec<&str>) -> Result<u32, ()> {
        if tokens.len() != 1 {
            return Err(());
        }

        tokens.first().ok_or(())?.parse::<u32>().map_err(|_| ())
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
        let token = Token::LineExecCount;
        let data: Option<String> = Some("1,2".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn sf_starts_entry() {
        let mut entry = EntryBuilder::new();
        let token = Token::SourceFile;
        let data: Option<String> = Some("/home/projects/file.txt".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
        assert!(entry.entry.is_some());
    }

    #[test]
    fn sf_invalid() {
        let token = Token::SourceFile;
        let cases: Vec<Option<String>> = vec![None, Some("".into())];

        let mut entry = EntryBuilder::new();

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn fn_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionName;
        let data: Option<String> = Some("1,function1".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn fn_invalid_cases() {
        let cases: Vec<Option<String>> = vec![
            None,
            Some("".into()),
            Some("1,".into()),
            Some(",function1".into()),
            Some("A,function1".into()),
        ];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionName;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnda_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionName;
        let data: Option<String> = Some("1,function1".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn fnda_invalid_cases() {
        let cases: Vec<Option<String>> = vec![
            None,
            Some("".into()),
            Some("1,".into()),
            Some(",function1".into()),
            Some("A,function1".into()),
        ];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionExecCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionCount;
        let data: Option<String> = Some("34".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn fnf_invalid_cases() {
        let cases: Vec<Option<String>> = vec![None, Some("".into()), Some("A".into())];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn fnh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionHitCount;
        let data: Option<String> = Some("23".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn fnh_invalid_cases() {
        let cases: Vec<Option<String>> = vec![None, Some("".into()), Some("A".into())];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::FunctionHitCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn da_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineExecCount;
        let data: Option<String> = Some("3,4".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn da_invalid_cases() {
        let cases: Vec<Option<String>> = vec![
            None,
            Some("".into()),
            Some("A,3".into()),
            Some("3,A".into()),
        ];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineExecCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn lf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineCount;
        let data: Option<String> = Some("34".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn lf_invalid_cases() {
        let cases: Vec<Option<String>> = vec![None, Some("".into()), Some("A".into())];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn lh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineHitCount;
        let data: Option<String> = Some("22".into());

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }

    #[test]
    fn lh_invalid_cases() {
        let cases: Vec<Option<String>> = vec![None, Some("".into()), Some("A".into())];

        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::LineHitCount;

        for case in cases {
            let result = entry.process_tokens(token.clone(), case.as_ref());
            assert!(result.is_err());
        }
    }

    #[test]
    fn end_of_record() {
        let mut entry = EntryBuilder::new();
        let sf_token = Token::SourceFile;
        let sf_data: Option<String> = Some("/home/projects/file.txt".into());
        entry.process_tokens(sf_token, sf_data.as_ref()).unwrap();

        let token = Token::EndOfRecord;
        let data: Option<String> = None;

        let result = entry.process_tokens(token, data.as_ref());
        assert!(result.is_ok());
    }
}
