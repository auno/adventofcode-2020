use std::str::FromStr;
use crate::eye_color::EyeColor::{Amber, Blue, Brown, Gray, Green, Hazel, Other};

#[derive(Debug)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other
}

#[derive(Debug)]
pub enum EyeColorParseError {
    InvalidFormat
}

impl FromStr for EyeColor {
    type Err = EyeColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Amber),
            "blu" => Ok(Blue),
            "brn" => Ok(Brown),
            "gry" => Ok(Gray),
            "grn" => Ok(Green),
            "hzl" => Ok(Hazel),
            "oth" => Ok(Other),
            _ => Err(EyeColorParseError::InvalidFormat)
        }
    }
}