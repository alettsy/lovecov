use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum LovecovError {
    ErrorAtLine(usize, Box<LovecovError>),
    UsageError,
    FileReadError,
    FileWriteError,
    InvalidStartToken,
    MissingToken,
    UnknownToken(String),
    RecordFormatError(String),
    RepeatedSourceTokenError,
    TreeError(String),
}

impl Display for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LovecovError::UsageError => {
                writeln!(f, "Usage error\n")?;
                writeln!(f, "Usage:")?;
                writeln!(f, "lovecov <file> <output (optional)>")?;
                writeln!(f, "\nOutput types:")?;
                writeln!(f, "- summary")?;
                writeln!(f, "- html")
            }
            LovecovError::FileReadError => writeln!(f, "Failed to read provided file"),
            LovecovError::MissingToken => writeln!(f, "Expected token, but found nothing"),
            LovecovError::RecordFormatError(token) => {
                writeln!(f, "Row for token '{}' formatted badly", token)
            }
            LovecovError::TreeError(message) => writeln!(f, "{}", message),
            LovecovError::UnknownToken(token) => {
                writeln!(f, "Unknown token '{}'", token)
            }
            LovecovError::ErrorAtLine(line, error) => {
                writeln!(f, "Error in file at line {}", line)?;
                Display::fmt(error, f)
            }
            LovecovError::InvalidStartToken => writeln!(f, "Invalid file entry start token"),
            LovecovError::RepeatedSourceTokenError => {
                writeln!(f, "Source file token repeated in entry")
            }
            LovecovError::FileWriteError => writeln!(f, "Failed to write to output file"),
        }
    }
}

impl Debug for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for LovecovError {}
