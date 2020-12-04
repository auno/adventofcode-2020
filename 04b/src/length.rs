use std::str::FromStr;
use regex::Regex;
use LengthUnit::{Centimeter, Inch};

#[derive(Debug)]
pub enum LengthUnit {
    Centimeter,
    Inch
}

#[derive(Debug)]
pub struct Length(usize, LengthUnit);

#[derive(Debug)]
pub enum LengthUnitParseError {
    InvalidFormat
}

impl FromStr for LengthUnit {
    type Err = LengthUnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Centimeter),
            "in" => Ok(Inch),
            _ => Err(LengthUnitParseError::InvalidFormat)
        }
    }
}

#[derive(Debug)]
pub enum LengthParseError {
    InvalidFormat
}

impl FromStr for Length {
    type Err = LengthParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LENGTH_FORMAT: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$")
                .unwrap();
        }

        if let Some(caps) = LENGTH_FORMAT.captures(s) {
            if let (Ok(value), Ok(unit)) = (
                usize::from_str(caps.name("value").unwrap().as_str()),
                LengthUnit::from_str(caps.name("unit").unwrap().as_str())
            ) {
                return Ok(Length(value, unit));
            }
        }

        Err(LengthParseError::InvalidFormat)
    }
}

impl Length {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn unit(&self) -> &LengthUnit {
        &self.1
    }
}
