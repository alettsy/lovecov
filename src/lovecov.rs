use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    entry::entry_builder::EntryBuilder, entry::file_tree::FileTree, errors::LovecovError,
    output::generator::Generator,
};

pub struct Lovecov {
    tree: FileTree,
    current_entry: Option<EntryBuilder>,
    output_generator: Generator,
    file: BufReader<File>,
}

impl Lovecov {
    pub fn from_args(args: Vec<String>) -> Result<Self, LovecovError> {
        let file_path = &args.get(1).ok_or(LovecovError::UsageError)?;

        let file = File::open(file_path).map_err(|_| LovecovError::FileReadError)?;
        let reader = BufReader::new(file);

        let generator_name = args.get(2).map(|s| s.as_str()).unwrap_or("summary");
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
            let tokens: Vec<&str> = line.split(":").collect();

            if tokens.is_empty() {
                continue;
            }

            let first_token = tokens
                .first()
                .ok_or(LovecovError::MissingToken(line_number))?
                .clone();

            if self.current_entry.is_none() {
                if first_token.eq("SF") {
                    self.current_entry = Some(EntryBuilder::new());
                }
            }

            if let Some(ref mut g) = self.current_entry {
                if !g.is_done() {
                    g.process_tokens(tokens, line_number)?;
                }

                if g.is_done() {
                    let entry = g.entry.clone().unwrap();
                    self.tree.add_file(entry)?;
                    self.current_entry = None;
                }
            }
        }

        self.tree.post_process();
        self.output_generator.generate(&self.tree);
        Ok(())
    }
}
