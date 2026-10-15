use crate::{
    entry::{
        file_data::FileData,
        parts::{Function, Line},
        record::Record,
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

    pub fn process_record(&mut self, record: Record) -> Result<(), LovecovError> {
        if self.entry.is_none() {
            if let Record::SourceFile(path) = record {
                let file_data =
                    FileData::new(&path).map_err(|_| LovecovError::InvalidStartToken)?;
                self.entry = Some(file_data);
                return Ok(());
            } else {
                return Err(LovecovError::InvalidStartToken);
            }
        }

        match record {
            Record::TestName => {
                // NOTE: ignoring TN because it is not that important
                Ok(())
            }
            Record::SourceFile(_) => Err(LovecovError::RepeatedSourceTokenError),
            Record::FunctionName(line, func) => {
                if let Some(entry) = &mut self.entry {
                    entry.functions.push(Function::new(&func, line));
                }
                Ok(())
            }
            Record::FunctionExecCount(count, func) => {
                if let Some(entry) = &mut self.entry {
                    let index_opt = entry.functions.iter().position(|f| f.name.eq(&func));

                    if index_opt.is_none() {
                        eprintln!("FNDA function {} not found -- ignoring", func);
                        return Ok(());
                    }

                    let index = index_opt.unwrap();
                    entry.functions[index].execute_count += count;
                }
                Ok(())
            }
            Record::FunctionCount(count) => {
                if let Some(entry) = &mut self.entry {
                    entry.function_count += count;
                }
                Ok(())
            }
            Record::FunctionHitCount(count) => {
                if let Some(entry) = &mut self.entry {
                    entry.function_hit_count += count;
                }
                Ok(())
            }
            Record::LineExecCount(line, count) => {
                if let Some(entry) = &mut self.entry {
                    entry.lines.push(Line::new(line, count));
                }
                Ok(())
            }
            Record::LineCount(count) => {
                if let Some(entry) = &mut self.entry {
                    entry.line_count += count;
                }
                Ok(())
            }
            Record::LineHitCount(count) => {
                if let Some(entry) = &mut self.entry {
                    entry.line_hit_count += count;
                }
                Ok(())
            }
            Record::EndOfRecord => {
                self.reached_end = true;
                Ok(())
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.reached_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_record_while_none_fails() {
        let mut entry = EntryBuilder::new();
        let record = Record::LineExecCount(1, 2);

        let result = entry.process_record(record);
        assert!(result.is_err());
    }

    #[test]
    fn sf_starts_entry() {
        let mut entry = EntryBuilder::new();
        let record = Record::SourceFile("/home/projects/file.txt".into());

        let result = entry.process_record(record);
        assert!(result.is_ok());
        assert!(entry.entry.is_some());
    }

    #[test]
    fn fn_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::FunctionName(1, "function1".into());

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn fnda_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::FunctionExecCount(4, "function1".into());

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn fnf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::FunctionCount(34);

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn fnh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::FunctionHitCount(11);

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn da_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::LineExecCount(3, 4);

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn lf_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::LineCount(34);

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn lh_valid() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::LineHitCount(22);

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }

    #[test]
    fn end_of_record() {
        let mut entry = EntryBuilder::new();
        let sf_record = Record::SourceFile("/home/projects/file.txt".into());
        entry.process_record(sf_record).unwrap();

        let record = Record::EndOfRecord;

        let result = entry.process_record(record);
        assert!(result.is_ok());
    }
}
