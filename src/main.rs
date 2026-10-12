use std::env;

use crate::{errors::LovecovError, lovecov::Lovecov};

mod errors;
mod entry;
mod lovecov;
mod output;

fn main() -> Result<(), LovecovError> {
    let args: Vec<String> = env::args().collect();

    let mut lovecov = Lovecov::from_args(args)?;
    lovecov.run()?;

    println!("\nAll finished!");

    Ok(())
}
