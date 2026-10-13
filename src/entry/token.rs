use std::str::FromStr;

use crate::{
    constants,
    entry::token::Token::{
        EndOfRecord, FunctionCount, FunctionExecCount, FunctionHitCount, FunctionName, LineCount,
        LineExecCount, LineHitCount, SourceFile, TestName,
    },
    errors::LovecovError,
};

#[derive(PartialEq, Clone)]
pub enum Token {
    TestName,
    SourceFile,
    FunctionName,
    FunctionExecCount,
    FunctionCount,
    FunctionHitCount,
    LineExecCount,
    LineCount,
    LineHitCount,
    EndOfRecord,
}

impl Token {
    pub fn into_err(&self) -> LovecovError {
        LovecovError::TokenFormatError(self.clone())
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            TestName => constants::TEST_NAME.to_string(),
            SourceFile => constants::SOURCE_FILE.to_string(),
            FunctionName => constants::FUNC_NAME.to_string(),
            FunctionExecCount => constants::FUN_EXEC_COUNT.to_string(),
            FunctionCount => constants::FUNC_COUNT.to_string(),
            FunctionHitCount => constants::FUNC_HIT_COUNT.to_string(),
            LineExecCount => constants::LINE_EXEC_COUNT.to_string(),
            LineCount => constants::LINE_COUNT.to_string(),
            LineHitCount => constants::LINE_HIT_COUNT.to_string(),
            EndOfRecord => constants::END_OF_RECORD.to_string(),
        }
    }
}

impl FromStr for Token {
    type Err = LovecovError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            constants::TEST_NAME => Ok(TestName),
            constants::SOURCE_FILE => Ok(SourceFile),
            constants::FUNC_NAME => Ok(FunctionName),
            constants::FUN_EXEC_COUNT => Ok(FunctionExecCount),
            constants::FUNC_COUNT => Ok(FunctionCount),
            constants::FUNC_HIT_COUNT => Ok(FunctionHitCount),
            constants::LINE_EXEC_COUNT => Ok(LineExecCount),
            constants::LINE_COUNT => Ok(LineCount),
            constants::LINE_HIT_COUNT => Ok(LineHitCount),
            constants::END_OF_RECORD => Ok(EndOfRecord),
            token => Err(LovecovError::UnknownToken(token.into())),
        }
    }
}
