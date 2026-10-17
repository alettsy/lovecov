use crate::{constants, errors::LovecovError};

pub enum Record {
    TestName,
    SourceFile(String),
    FunctionName(u32, String),
    FunctionExecCount(u32, String),
    FunctionCount(u32),
    FunctionHitCount(u32),
    LineExecCount(u32, u32),
    LineCount(u32),
    LineHitCount(u32),
    BranchCoverageData,
    BranchCount,
    BranchHitCount,
    End,
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
            return Ok(Record::End);
        }

        if token_str == constants::TEST_NAME {
            return Ok(Record::TestName);
        }

        let data = data
            .ok_or(LovecovError::RecordFormatError(token.into()))?
            .to_string();

        let data =
            Self::extract_string(data).ok_or(LovecovError::RecordFormatError(token.into()))?;

        match token_str.as_str() {
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
            constants::BRANCH_DATA => Ok(Record::BranchCoverageData),
            constants::BRANCH_COUNT => Ok(Record::BranchCount),
            constants::BRANCH_HIT_COUNT => Ok(Record::BranchHitCount),
            token => Err(LovecovError::UnknownToken(token.into())),
        }
    }

    fn process_num_string(data: String) -> Result<(u32, String), ()> {
        let split: Vec<&str> = data.split(constants::DATA_SPLIT).collect();

        if split.len() != 2 {
            return Err(());
        }

        let num: u32 = split.first().unwrap().parse().map_err(|_| ())?;
        let string = Self::extract_string(split.last().unwrap().to_string()).ok_or(())?;

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

    fn extract_string(data: String) -> Option<String> {
        let result = data.trim().to_string();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sf_valid() {
        let cases = [
            "SF:file",
            "SF:file.txt",
            "SF:sub/folder/file.txt",
            "SF:/home/project/file.txt",
        ];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn sf_invalid() {
        let cases = ["SF:", "SF: "];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn lh_valid() {
        let cases = ["LH:0", "LH:2"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn lh_invalid() {
        let cases = ["LH:", "LH: ", "LH:-1", "LH:asdas"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn lf_valid() {
        let cases = ["LF:0", "LF:2"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn lf_invalid() {
        let cases = ["LF:", "LF: ", "LF:-1", "LF:asdas"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn fnf_valid() {
        let cases = ["FNF:0", "FNF:2"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn fnf_invalid() {
        let cases = ["FNF:", "FNF: ", "FNF:-1", "FNF:asdas"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn fnh_valid() {
        let cases = ["FNH:0", "FNH:2"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn fnh_invalid() {
        let cases = ["FNH:", "FNH: ", "FNH:-1", "FNH:asdas"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn da_valid() {
        let cases = ["DA:1,2", "DA:111,2222", "DA:2,-"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn da_invalid() {
        let cases = [
            "DA:",
            "DA: ",
            "DA:,",
            "DA:1",
            "DA:1,",
            "DA:,2",
            "DA:apple,2",
            "DA:3,apple",
            "DA:-,4",
        ];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }

    #[test]
    fn fnda_valid() {
        let cases = ["FNDA:1,func", "FNDA:111,function_that_is_long"];

        for case in cases {
            let record = Record::from_line(case);
            assert!(record.is_ok());
        }
    }

    #[test]
    fn fnda_invalid() {
        let cases = [
            "FNDA:",
            "FNDA: ",
            "FNDA:1",
            "FNDA:func",
            "FNDA:,",
            "FNDA:1,",
            "FNDA:,func",
            "FNDA:func,2",
            "FNDA:-,4",
        ];

        for case in cases {
            println!("'{}'", case);
            let record = Record::from_line(case);
            assert!(record.is_err());
        }
    }
}
