use crate::files::{FileData, Function, Line};

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

    pub fn process_tokens(&mut self, tokens: Vec<&str>) {
        let first_token = tokens.first().unwrap();

        if self.entry.is_none() {
            if first_token.eq(&"SF") {
                if tokens.len() != 2 {
                    panic!("Invalid SF format");
                }

                let file_path = tokens[1];
                self.entry = Some(FileData::new(file_path));
                return;
            } else {
                panic!("Invalid initial entry token: {}", first_token);
            }
        }

        match *first_token {
            "TN" => {
                // NOTE: ignoring TN because it is not that important
            }
            "SF" => panic!("Multiple SF entries found"),
            "FN" => self.process_function_name(tokens),
            "FNDA" => self.process_function_execute(tokens),
            "FNF" => {
                let count = Self::extract_count(tokens).expect("Invalid FNF");

                if let Some(entry) = &mut self.entry {
                    entry.function_count += count;
                }
            }
            "FNH" => {
                let count = Self::extract_count(tokens).expect("Invalid FNH");

                if let Some(entry) = &mut self.entry {
                    entry.function_hit_count += count;
                }
            }
            "DA" => self.process_line_execute_count(tokens),
            "LF" => {
                let count = Self::extract_count(tokens).expect("Invalid LF");

                if let Some(entry) = &mut self.entry {
                    entry.line_count += count;
                }
            }
            "LH" => {
                let count = Self::extract_count(tokens).expect("Invalid LH");

                if let Some(entry) = &mut self.entry {
                    entry.line_hit_count += count;
                }
            }
            "end_of_record" => self.reached_end = true,
            _ => println!("Skipping unknown token: {}", *first_token),
        };
    }

    fn process_line_execute_count(&mut self, tokens: Vec<&str>) {
        let parts = Self::extract_two_parts(tokens).expect("Invalid DA format");

        let line_number: u32 = parts
            .first()
            .expect("Line number missing in DA")
            .parse()
            .expect("Line number invalid in DA");

        let hit_count_str = parts.last().expect("Hit count missing in DA");

        let hit_count = if hit_count_str.eq(&"-") {
            1
        } else {
            hit_count_str.parse().expect("Hit count invalid in DA")
        };

        if let Some(entry) = &mut self.entry {
            entry.lines.push(Line::new(line_number, hit_count));
        }
    }

    fn process_function_name(&mut self, tokens: Vec<&str>) {
        let parts = Self::extract_two_parts(tokens).expect("Invalid FN format");

        let line_number: u32 = parts
            .first()
            .expect("Line number missing in FN")
            .parse()
            .expect("Line number invalid in FN");
        let function_name = parts.last().expect("Function name missing in FN");

        if let Some(entry) = &mut self.entry {
            entry
                .functions
                .push(Function::new(function_name, line_number));
        }
    }

    fn process_function_execute(&mut self, tokens: Vec<&str>) {
        let parts = Self::extract_two_parts(tokens).expect("Invalid FNDA format");

        let count: u32 = parts
            .first()
            .expect("Count missing in FNDA")
            .parse()
            .expect("Count invalid in FNDA");
        let function_name = parts.last().expect("Function name missing in FNDA");

        if let Some(entry) = &mut self.entry {
            let index_opt = entry
                .functions
                .iter()
                .position(|f| f.name.eq(function_name));

            if index_opt.is_none() {
                println!("FNDA function {} not found", function_name);
                return;
            }

            let index = index_opt.unwrap();
            entry.functions[index].execute_count += count;
        }
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
