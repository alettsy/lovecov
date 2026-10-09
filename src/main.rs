use std::{env, fs};

use crate::{entry_builder::EntryBuilder, files::FileTree, generator::Generator};

mod entry_builder;
mod files;
mod generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args.get(1).ok_or("No file provided")?;

    let contents = fs::read_to_string(file_path)?;

    let generator_name = args.get(2).map(|s| s.as_str()).unwrap_or("summary");
    let generator = Generator::from_str(generator_name);

    println!("Processing...\n");

    let mut tree = FileTree::new();

    let lines = contents.lines();

    let mut builder: Option<EntryBuilder> = None;

    for line in lines {
        let tokens: Vec<&str> = line.split(":").collect();

        if tokens.is_empty() {
            continue;
        }

        let first_token = tokens.first().unwrap().clone();

        if builder.is_none() {
            if first_token.eq("SF") {
                builder = Some(EntryBuilder::new());
            }
        }

        if let Some(ref mut g) = builder {
            if !g.is_done() {
                g.process_tokens(tokens);
            }

            if g.is_done() {
                let entry = g.entry.clone().unwrap();
                tree.add_file(entry);
                builder = None;
            }
        }
    }

    generator.generate(&tree);

    println!("\nAll finished!");

    Ok(())
}
