use crate::length::{Length, LengthUnit};
use crate::hair_color::HairColor;
use crate::eye_color::EyeColor;
use crate::passport_id::PassportId;

#[derive(Debug)]
pub struct PartialPassport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<Length>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<PassportId>,
    country_id: Option<String>
}

impl PartialPassport {
    pub fn new() -> PartialPassport {
        PartialPassport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        }
    }

    pub fn is_valid(&self) -> bool {
        match &self.birth_year {
            Some(year) if year < &1920 => return false,
            Some(year) if year > &2002 => return false,
            None => return false,
            _ => {}
        }

        match &self.issue_year {
            Some(year) if year < &2010 => return false,
            Some(year) if year > &2020 => return false,
            None => return false,
            _ => {}
        }

        match &self.expiration_year {
            Some(year) if year < &2020 => return false,
            Some(year) if year > &2030 => return false,
            None => return false,
            _ => {}
        }

        if let Some(height) = &self.height {
            match height.unit() {
                LengthUnit::Centimeter => { if height.value() < 150 || height.value() > 193 { return false }; }
                LengthUnit::Inch => { if height.value() < 59 || height.value() > 76 { return false }; }
            }
        } else {
            return false;
        }

        if self.hair_color.is_none() { return false; }
        if self.eye_color.is_none() { return false; }
        if self.passport_id.is_none() { return false; }

        true
    }
}

impl From<&[String]> for PartialPassport {
    fn from(input: &[String]) -> PartialPassport {
        let mut pp = PartialPassport::new();

        input.iter()
            .flat_map(|line| line.split(" "))
            .map(|kv| {
                let mut kv = kv.split(":");
                (kv.next().unwrap(), kv.next().unwrap())
            })
            .for_each(|(key, value)| match key {
                "byr" => {
                    if let Ok(birth_year) = value.parse() {
                        pp.birth_year = Some(birth_year);
                    }
                }
                "iyr" => {
                    if let Ok(issue_year) = value.parse() {
                        pp.issue_year = Some(issue_year);
                    }
                }
                "eyr" => {
                    if let Ok(expiration_year) = value.parse() {
                        pp.expiration_year = Some(expiration_year);
                    }
                }
                "hgt" => {
                    if let Ok(height) = value.parse() {
                        pp.height = Some(height);
                    }
                }
                "hcl" => {
                    if let Ok(hair_color) = value.parse() {
                        pp.hair_color = Some(hair_color);
                    }
                }
                "ecl" => {
                    if let Ok(eye_color) = value.parse() {
                        pp.eye_color = Some(eye_color);
                    }
                }
                "pid" => {
                    if let Ok(passport_id) = value.parse() {
                        pp.passport_id = Some(passport_id);
                    }
                }
                "cid" => { pp.country_id = Some(String::from(value)); }
                _ => { panic!("Unknown key: {}", key); }
            });

        pp
    }
}