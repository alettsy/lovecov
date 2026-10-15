use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    constants,
    entry::{entry_builder::EntryBuilder, file_tree::FileTree, record::Record},
    errors::LovecovError,
    output::generator::Generator,
};

pub struct Lovecov<R> {
    tree: FileTree,
    current_entry: Option<EntryBuilder>,
    output_generator: Generator,
    file: R,
}

impl Lovecov<BufReader<File>> {
    pub fn from_args(args: Vec<String>) -> Result<Self, LovecovError> {
        let file_path = &args.get(1).ok_or(LovecovError::UsageError)?;

        let file = File::open(file_path).map_err(|_| LovecovError::FileReadError)?;
        let reader = BufReader::new(file);

        Self::from_reader(args, reader)
    }
}

impl<R: BufRead> Lovecov<R> {
    fn from_reader(args: Vec<String>, reader: R) -> Result<Self, LovecovError> {
        let generator_name = args
            .get(2)
            .map(|s| s.as_str())
            .unwrap_or(constants::DEFAULT_OUTPUT);
        let generator = Generator::from_str(generator_name)?;

        Ok(Self {
            tree: FileTree::new(),
            current_entry: None,
            output_generator: generator,
            file: reader,
        })
    }

    pub fn run(&mut self) -> Result<(), LovecovError> {
        let file = &mut self.file;

        for (i, line) in file.lines().enumerate() {
            let line_number = i + 1;
            let line = line.map_err(|_| LovecovError::FileReadError)?;

            let record = Record::from_line(&line)
                .map_err(|e| LovecovError::ErrorAtLine(line_number, Box::new(e)))?;

            if self.current_entry.is_none()
                && let Record::SourceFile(_) = record
            {
                self.current_entry = Some(EntryBuilder::new());
            }

            if let Some(ref mut g) = self.current_entry {
                if !g.is_done() {
                    g.process_record(record)
                        .map_err(|e| LovecovError::ErrorAtLine(line_number, Box::new(e)))?;
                }

                if g.is_done() {
                    let entry = g
                        .entry
                        .clone()
                        .expect("Failed to get entry when end reached");
                    self.tree.add_file(entry)?;
                    self.current_entry = None;
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
    fn empty_args_fails() {
        let args: Vec<String> = vec![];

        let lovecov = Lovecov::from_args(args);

        assert!(lovecov.is_err());
    }

    #[test]
    fn missing_file_arg_fails() {
        let args: Vec<String> = vec!["lovecov".into()];

        let lovecov = Lovecov::from_args(args);

        assert!(lovecov.is_err());
    }

    #[test]
    fn not_found_file_fails() {
        let args: Vec<String> = vec!["lovecov".into(), "abcdefghijklmnop".into()];

        let lovecov = Lovecov::from_args(args);

        assert!(lovecov.is_err());
    }

    #[test]
    fn empty_output_defaults_to_summary() {
        let args: Vec<String> = vec!["lovecov".into(), "file.txt".into()];

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
    fn invalid_output_fails() {
        let args: Vec<String> = vec!["lovecov".into(), "file.txt".into(), "apples".into()];

        let mock_data = "SF:src/lib.rs\nDA:1,1\nEND_OF_RECORD\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        assert!(lovecov.is_err());
    }

    #[test]
    fn provided_output_is_used() {
        let args: Vec<String> = vec!["lovecov".into(), "file.txt".into(), "html".into()];

        let mock_data = "SF:src/lib.rs\nDA:1,1\nEND_OF_RECORD\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        assert!(lovecov.is_ok());
        assert!(matches!(lovecov.unwrap().output_generator, Generator::Html));
    }

    #[test]
    fn bad_file_fails() {
        let args: Vec<String> = vec!["lovecov".into(), "file.txt".into()];

        let mock_data = "SF:/test/file.txt\nDA:aaaaa\nend_of_record";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        let result = lovecov.unwrap().run();

        assert!(result.is_err());
    }

    #[test]
    fn valid_file_passes() {
        let args: Vec<String> = vec!["lovecov".into(), "file.txt".into()];

        let mock_data = "SF:src/lib.rs\nDA:1,1\nend_of_record\n";
        let cursor = Cursor::new(mock_data.as_bytes());

        let lovecov = Lovecov::from_reader(args, cursor);

        let result = lovecov.unwrap().run();

        assert!(result.is_ok());
    }
}
