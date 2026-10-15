use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum LovecovError {
    ErrorAtLine(usize, Box<LovecovError>),
    FileReadError,
    FileWriteError,
    InvalidStartToken,
    MissingToken,
    UnknownToken(String),
    RecordFormatError(String),
    RepeatedSourceTokenError,
    TreeError(String),
}

impl LovecovError {
    pub fn at_line(self, line: usize) -> LovecovError {
        LovecovError::ErrorAtLine(line, Box::new(self))
    }
}

impl Display for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LovecovError::FileReadError => writeln!(f, "Failed to read provided file"),
            LovecovError::MissingToken => writeln!(f, "Expected token, but found nothing"),
            LovecovError::RecordFormatError(token) => {
                writeln!(f, "Record for token '{}' invalid", token)
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
