use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LovecovError {
    FileNotFoundError,
    FileReadError,
}

impl Display for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LovecovError::FileNotFoundError => write!(f, "File not found"),
            LovecovError::FileReadError => write!(f, "Failed to read in file"),
        }
    }
}

impl Error for LovecovError {}
