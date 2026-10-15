use crate::{constants, errors::LovecovError};

pub enum Record {
    TestName(String),
    SourceFile(String),
    FunctionName(u32, String),
    FunctionExecCount(u32, String),
    FunctionCount(u32),
    FunctionHitCount(u32),
    LineExecCount(u32, u32),
    LineCount(u32),
    LineHitCount(u32),
    EndOfRecord,
}

impl Record {
    pub fn from_line(line: &str) -> Result<Self, LovecovError> {
        let parts: Vec<&str> = line.split(constants::TOKEN_SPLIT).collect();

        if parts.is_empty() {
            return Err(LovecovError::MissingToken);
        }

        let token = parts.first().copied().ok_or(LovecovError::MissingToken)?;

        let data = parts.get(1).copied();

        Self::from_token_and_data(token, data)
    }

    fn from_token_and_data(token: &str, data: Option<&str>) -> Result<Self, LovecovError> {
        let token_str = token.to_string().to_lowercase();

        if token_str == constants::END_OF_RECORD {
            return Ok(Record::EndOfRecord);
        }

        let data = data
            .ok_or(LovecovError::RecordFormatError(token.into()))?
            .to_string();

        match token_str.as_str() {
            constants::TEST_NAME => Ok(Record::TestName(data)),
            constants::SOURCE_FILE => Ok(Record::SourceFile(data)),
            constants::FUNC_NAME => {
                let values = Self::process_num_string(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::FunctionName(values.0, values.1))
            }
            constants::FUN_EXEC_COUNT => {
                let values = Self::process_num_string(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::FunctionExecCount(values.0, values.1))
            }
            constants::FUNC_COUNT => {
                let count = Self::process_num(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::FunctionCount(count))
            }
            constants::FUNC_HIT_COUNT => {
                let count = Self::process_num(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::FunctionHitCount(count))
            }
            constants::LINE_EXEC_COUNT => {
                let values = Self::process_double_num(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::LineExecCount(values.0, values.1))
            }
            constants::LINE_COUNT => {
                let count = Self::process_num(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::LineCount(count))
            }
            constants::LINE_HIT_COUNT => {
                let count = Self::process_num(data)
                    .map_err(|_| LovecovError::RecordFormatError(token.into()))?;
                Ok(Record::LineHitCount(count))
            }
            token => Err(LovecovError::UnknownToken(token.into())),
        }
    }

    fn process_num_string(data: String) -> Result<(u32, String), ()> {
        let split: Vec<&str> = data.split(constants::DATA_SPLIT).collect();

        if split.len() != 2 {
            return Err(());
        }

        let num: u32 = split.first().unwrap().parse().map_err(|_| ())?;
        let string = split.last().unwrap().to_string();

        Ok((num, string))
    }

    fn process_double_num(data: String) -> Result<(u32, u32), ()> {
        let split: Vec<&str> = data.split(constants::DATA_SPLIT).collect();

        if split.len() != 2 {
            return Err(());
        }

        let num1: u32 = split.first().unwrap().parse().map_err(|_| ())?;

        let last_part = split.last().unwrap();
        let num2: u32 = if *last_part == constants::HIT_EMPTY {
            0
        } else {
            last_part.parse().map_err(|_| ())?
        };

        Ok((num1, num2))
    }

    fn process_num(data: String) -> Result<u32, ()> {
        data.parse::<u32>().map_err(|_| ())
    }
}
