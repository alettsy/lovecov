use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum LovecovError {
    ErrorAtLine(usize, Box<LovecovError>),
    FileRead,
    FileWrite,
    InvalidFilePath,
    MissingTag,
    UnknownTag(String),
    InvalidStartTag,
    RepeatedSourceTag,
    RecordFormat(String),
}

impl LovecovError {
    pub fn at_line(self, line: usize) -> LovecovError {
        LovecovError::ErrorAtLine(line, Box::new(self))
    }
}

impl Display for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LovecovError::FileRead => writeln!(f, "Failed to read provided file"),
            LovecovError::MissingTag => writeln!(f, "Expected tag, but found nothing"),
            LovecovError::RecordFormat(tag) => {
                writeln!(f, "Record for tag '{}' invalid", tag)
            }
            LovecovError::UnknownTag(tag) => {
                writeln!(f, "Unknown tag '{}'", tag)
            }
            LovecovError::ErrorAtLine(line, error) => {
                writeln!(f, "Error in file at line {}", line)?;
                Display::fmt(error, f)
            }
            LovecovError::InvalidStartTag => writeln!(f, "Invalid file record start tag"),
            LovecovError::RepeatedSourceTag => {
                writeln!(f, "Source file tag repeated in record")
            }
            LovecovError::FileWrite => writeln!(f, "Failed to write to output file"),
            LovecovError::InvalidFilePath => writeln!(f, "Invalid output path provided"),
        }
    }
}

impl Debug for LovecovError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for LovecovError {}
