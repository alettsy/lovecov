use crate::{
    parser::{
        file_data::FileData,
        parts::{Function, Line},
        field::Field,
    },
    errors::LovecovError,
};

pub struct RecordBuilder {
    pub record: Option<FileData>,
    reached_end: bool,
}

impl RecordBuilder {
    pub fn new() -> Self {
        Self {
            record: None,
            reached_end: false,
        }
    }

    pub fn process_field(&mut self, field: Field) -> Result<(), LovecovError> {
        if self.record.is_none() {
            if let Field::SourceFile(path) = field {
                let file_data =
                    FileData::new(&path).map_err(|_| LovecovError::InvalidStartTag)?;
                self.record = Some(file_data);
                return Ok(());
            } else {
                return Err(LovecovError::InvalidStartTag);
            }
        }

        match field {
            Field::SourceFile(_) => Err(LovecovError::RepeatedSourceTag),
            Field::FunctionName(line, func) => {
                if let Some(record) = &mut self.record {
                    record.functions.push(Function::new(&func, line));
                }
                Ok(())
            }
            Field::FunctionExecCount(count, func) => {
                if let Some(record) = &mut self.record {
                    let index_opt = record.functions.iter().position(|f| f.name.eq(&func));

                    if index_opt.is_none() {
                        eprintln!("FNDA function {} not found -- ignoring", func);
                        return Ok(());
                    }

                    let index = index_opt.unwrap();
                    record.functions[index].execute_count += count;
                }
                Ok(())
            }
            Field::FunctionCount(count) => {
                if let Some(record) = &mut self.record {
                    record.function_count += count;
                }
                Ok(())
            }
            Field::FunctionHitCount(count) => {
                if let Some(record) = &mut self.record {
                    record.function_hit_count += count;
                }
                Ok(())
            }
            Field::LineExecCount(line, count) => {
                if let Some(record) = &mut self.record {
                    record.lines.push(Line::new(line, count));
                }
                Ok(())
            }
            Field::LineCount(count) => {
                if let Some(record) = &mut self.record {
                    record.line_count += count;
                }
                Ok(())
            }
            Field::LineHitCount(count) => {
                if let Some(record) = &mut self.record {
                    record.line_hit_count += count;
                }
                Ok(())
            }
            Field::End => {
                self.reached_end = true;
                Ok(())
            }
            // NOTE: ignoring the below 4 as their data is not currently used
            Field::TestName => Ok(()),
            Field::BranchCoverageData => Ok(()),
            Field::BranchCount => Ok(()),
            Field::BranchHitCount => Ok(()),
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
    fn any_field_while_none_fails() {
        let mut record = RecordBuilder::new();
        let field = Field::LineExecCount(1, 2);

        let result = record.process_field(field);
        assert!(result.is_err());
    }

    #[test]
    fn sf_starts_record() {
        let mut record = RecordBuilder::new();
        let field = Field::SourceFile("/home/projects/file.txt".into());

        let result = record.process_field(field);
        assert!(result.is_ok());
        assert!(record.record.is_some());
    }

    #[test]
    fn fn_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::FunctionName(1, "function1".into());

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn fnda_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::FunctionExecCount(4, "function1".into());

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn fnf_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::FunctionCount(34);

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn fnh_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::FunctionHitCount(11);

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn da_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::LineExecCount(3, 4);

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn lf_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::LineCount(34);

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn lh_valid() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::LineHitCount(22);

        let result = record.process_field(field);
        assert!(result.is_ok());
    }

    #[test]
    fn end_of_field() {
        let mut record = RecordBuilder::new();
        let sf_field = Field::SourceFile("/home/projects/file.txt".into());
        record.process_field(sf_field).unwrap();

        let field = Field::End;

        let result = record.process_field(field);
        assert!(result.is_ok());
    }
}
