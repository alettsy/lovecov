use clap::Parser;

use crate::{cli::LovecovArgs, errors::LovecovError, lovecov::Lovecov};

mod cli;
mod constants;
mod entry;
mod errors;
mod lovecov;
mod output;
mod utils;

fn main() -> Result<(), LovecovError> {
    let args = LovecovArgs::parse();

    let mut lovecov = Lovecov::from_args(args)?;
    lovecov.run()?;

    Ok(())
}
