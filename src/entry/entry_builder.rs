use crate::{
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
            .ok_or(LovecovError::MissingToken(line_number))?;

        if self.entry.is_none() {
            if first_token.eq(&"SF") {
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

        match *first_token {
            "TN" => {
                // NOTE: ignoring TN because it is not that important
                Ok(())
            }
            "SF" => Err(LovecovError::TokenFormatError(
                line_number,
                String::from("Repeated SF in entry"),
            )),
            "FN" => {
                self.process_function_name(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            "FNDA" => {
                self.process_function_execute(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            "FNF" => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid FNF".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.function_count += count;
                }

                Ok(())
            }
            "FNH" => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid FNH".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.function_hit_count += count;
                }

                Ok(())
            }
            "DA" => {
                self.process_line_execute_count(tokens)
                    .map_err(|s| LovecovError::TokenFormatError(line_number, s))?;
                Ok(())
            }
            "LF" => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid LF".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.line_count += count;
                }

                Ok(())
            }
            "LH" => {
                let count = Self::extract_count(tokens).map_err(|_| {
                    LovecovError::TokenFormatError(line_number, "Invalid LH".into())
                })?;

                if let Some(entry) = &mut self.entry {
                    entry.line_hit_count += count;
                }

                Ok(())
            }
            "end_of_record" => {
                self.reached_end = true;
                Ok(())
            }
            _ => Err(LovecovError::UnknownToken(
                line_number,
                String::from(*first_token),
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
