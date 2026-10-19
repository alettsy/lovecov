use clap::Parser;

use crate::{cli::args::LovecovArgs, errors::LovecovError, lovecov::Lovecov};

mod cli;
mod constants;
mod errors;
mod lovecov;
mod output;
mod record;
mod utils;

fn main() -> Result<(), LovecovError> {
    let args = LovecovArgs::parse();

    let mut lovecov = Lovecov::from_args(args)?;
    lovecov.run()?;

    Ok(())
}
