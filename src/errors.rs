use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum LovecovError {
    UsageError,
    FileReadError,
    MissingToken(usize),
    UnknownToken(usize, String),
    TokenFormatError(usize, String),
    TreeError(String),
}

impl Display for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LovecovError::UsageError => write!(
                f,
                "Invalid Usage\n\nlovecov <file> <optional output type>\n\nOutput types:\nsummary\nhtml"
            ),
            LovecovError::FileReadError => write!(f, "Failed to read provided file"),
            LovecovError::MissingToken(line) => write!(f, "Expected token at line {}", line),
            LovecovError::TokenFormatError(line, message) => {
                write!(f, "Token Format Error at line {}\n{}", line, message)
            }
            LovecovError::TreeError(message) => write!(f, "{}", message),
            LovecovError::UnknownToken(line, token) => {
                write!(f, "Unknown token '{}' at line {}", token, line)
            }
        }
    }
}

impl Debug for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for LovecovError {}
