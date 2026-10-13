use std::env;

use crate::{errors::LovecovError, lovecov::Lovecov};

mod entry;
mod errors;
mod lovecov;
mod output;
mod constants;

fn main() -> Result<(), LovecovError> {
    let args: Vec<String> = env::args().collect();

    let mut lovecov = Lovecov::from_args(args)?;
    lovecov.run()?;

    Ok(())
}
