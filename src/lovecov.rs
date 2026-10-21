use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    cli::{args::LovecovArgs, generator_type::GeneratorType},
    errors::LovecovError,
    output::generator::Generator,
    parser::{
        field::Field, file_tree::FileTree, ignore::IgnoreCondition, record_builder::RecordBuilder,
    },
};

pub struct Lovecov<R> {
    tree: FileTree,
    current_record: Option<RecordBuilder>,
    output_generator: Generator,
    ignorer: Option<IgnoreCondition>,
    file: R,
}

impl Lovecov<BufReader<File>> {
    pub fn from_args(args: LovecovArgs) -> Result<Self, LovecovError> {
        let file = File::open(&args.file_path).map_err(|_| LovecovError::FileRead)?;
        let reader = BufReader::new(file);

        Self::from_reader(args, reader)
    }
}

impl<R: BufRead> Lovecov<R> {
    fn from_reader(args: LovecovArgs, reader: R) -> Result<Self, LovecovError> {
        let generator_type = args.output.clone().unwrap_or(GeneratorType::Summary);
        let generator = Generator::from_type_and_args(&args, generator_type);

        let ignorer = if let Some(s) = args.ignore {
            Some(IgnoreCondition::from_str(&s)?)
        } else {
            None
        };

        Ok(Self {
            tree: FileTree::new(),
            current_record: None,
            output_generator: generator,
            file: reader,
            ignorer,
        })
    }

    pub fn run(&mut self) -> Result<(), LovecovError> {
        let file = &mut self.file;

        for (i, line) in file.lines().enumerate() {
            let line_number = i + 1;
            let line = line.map_err(|_| LovecovError::FileRead)?;

            if line.is_empty() {
                continue;
            }

            let record = Field::from_line(&line).map_err(|e| e.at_line(line_number))?;

            if self.current_record.is_none()
                && let Field::SourceFile(_) = record
            {
                self.current_record = Some(RecordBuilder::new());
            }

            if let Some(ref mut g) = self.current_record {
                if !g.is_done() {
                    g.process_field(record)
                        .map_err(|e| e.at_line(line_number))?;
                }

                if g.is_done() {
                    let record = g
                        .record
                        .clone()
                        .expect("Failed to get record when end reached");

                    let should_ignore = if let Some(ignorer) = &self.ignorer {
                        ignorer.should_ignore(&record.full_path())
                    } else {
                        false
                    };

                    if !should_ignore {
                        self.tree.add_file(record)?;
                    }

                    self.current_record = None;
                }
            }
        }

        self.tree.post_process();
        self.output_generator.generate(&self.tree)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn not_found_file_fails() {
        let args = LovecovArgs {
            file_path: "adasdasdasdasds".into(),
            output: None,
            output_path: None,
            ignore: None,
        };

        let lovecov = Lovecov::from_args(args);

        assert!(lovecov.is_err());
    }

    #[test]
    fn empty_output_defaults_to_summary() {
        let args = LovecovArgs {
            file_path: "file.txt".into(),
            output: None,
            output_path: None,
            ignore: None,
        };

        let mock_data = "SF:src/lib.rs\nDA:1,1\nEND_OF_RECORD\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        assert!(lovecov.is_ok());
        assert!(matches!(
            lovecov.unwrap().output_generator,
            Generator::Summary
        ));
    }

    #[test]
    fn provided_output_is_used() {
        let args = LovecovArgs {
            file_path: "file.txt".into(),
            output: Some(GeneratorType::Html),
            output_path: None,
            ignore: None,
        };

        let mock_data = "SF:src/lib.rs\nDA:1,1\nEND_OF_RECORD\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        assert!(lovecov.is_ok());
        assert!(matches!(
            lovecov.unwrap().output_generator,
            Generator::Html(_)
        ));
    }

    #[test]
    fn bad_file_fails() {
        let args = LovecovArgs {
            file_path: "file.txt".into(),
            output: None,
            output_path: None,
            ignore: None,
        };

        let mock_data = "SF:/test/file.txt\nDA:aaaaa\nend_of_record";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        let result = lovecov.unwrap().run();

        assert!(result.is_err());
    }

    #[test]
    fn valid_file_passes() {
        let args = LovecovArgs {
            file_path: "file.txt".into(),
            output: None,
            output_path: None,
            ignore: None,
        };

        let mock_data = "SF:src/lib.rs\nDA:1,1\nend_of_record\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        let result = lovecov.unwrap().run();

        assert!(result.is_ok());
    }
}
