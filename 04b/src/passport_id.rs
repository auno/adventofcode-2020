extern crate derive_more;

use derive_more::Display;
use std::str::FromStr;
use regex::Regex;

#[derive(Display, Debug)]
pub struct PassportId(String);

#[derive(Debug)]
pub enum PassportIdParseError {
    InvalidFormat
}

impl FromStr for PassportId {
    type Err = PassportIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format: Regex = Regex::new(r"^[0-9]{9}$").unwrap();

        if format.is_match(s) {
            return Ok(PassportId(s.to_string()));
        }

        Err(PassportIdParseError::InvalidFormat)
    }
}