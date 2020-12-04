extern crate derive_more;

use derive_more::Display;
use std::str::FromStr;
use regex::Regex;

#[derive(Display, Debug)]
pub struct HairColor(String);

#[derive(Debug)]
pub enum HairColorParseError {
    InvalidFormat
}

impl FromStr for HairColor {
    type Err = HairColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

        if format.is_match(s) {
            return Ok(HairColor(s.to_string()));
        }

        Err(HairColorParseError::InvalidFormat)
    }
}