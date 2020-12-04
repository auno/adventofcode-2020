mod length;
mod hair_color;
mod eye_color;
mod passport_id;
mod passport;

use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;

use crate::passport::PartialPassport;

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
