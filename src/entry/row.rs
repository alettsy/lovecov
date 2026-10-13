use std::str::FromStr;

use crate::{constants, entry::token::Token, errors::LovecovError};

pub struct Row {
    pub token: Token,
    pub data: Option<String>,
}

impl FromStr for Row {
    type Err = LovecovError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(constants::TOKEN_SPLIT).collect();

        if parts.is_empty() {
            return Err(LovecovError::MissingToken);
        }

        let token_str = parts
            .first()
            .copied()
            .ok_or(LovecovError::MissingToken)?;

        let token = Token::from_str(token_str)?;

        let data = if parts.len() == 2 {
            Some(parts[1].into())
        } else {
            None
        };
        
        Ok(Row { token, data })
    }
}
