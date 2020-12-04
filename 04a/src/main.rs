use std::io::{self, BufRead};

#[derive(Debug)]
struct PartialPassport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
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
        [
            &self.birth_year,
            &self.issue_year,
            &self.expiration_year,
            &self.height,
            &self.hair_color,
            &self.eye_color,
            &self.passport_id,
        ].iter().all(|value| value.is_some())
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
            .for_each(|(key, value)| {
                match key {
                    "byr" => { pp.birth_year = Some(String::from(value)); }
                    "iyr" => { pp.issue_year = Some(String::from(value)); }
                    "eyr" => { pp.expiration_year = Some(String::from(value)); }
                    "hgt" => { pp.height = Some(String::from(value)); }
                    "hcl" => { pp.hair_color = Some(String::from(value)); }
                    "ecl" => { pp.eye_color = Some(String::from(value)); }
                    "pid" => { pp.passport_id = Some(String::from(value)); }
                    "cid" => { pp.country_id = Some(String::from(value)); }
                    _ => { panic!("Unknown key: {}", key); }
                }
            });

        pp
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let chunks: Vec<&[String]> = lines.split(|line| line.trim().is_empty()).collect();
    let valid = chunks.iter()
        .map(|chunk| chunk.to_owned().into())
        .filter(|pp: &PartialPassport| pp.is_valid())
        .count();

    println!("{}", valid);
}
