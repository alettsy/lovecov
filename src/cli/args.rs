use clap::Parser;

use crate::cli::generator_type::GeneratorType;


#[derive(Parser, Debug)]
#[command(name = "lovecov")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "alettsy")]
#[command(about = "A simple, cross-platform lcov parser for generating various output formats")]
pub struct LovecovArgs {
    pub file_path: String,
    pub output: Option<GeneratorType>,

    #[arg(short, long)]
    pub output_path: Option<String>,
}
