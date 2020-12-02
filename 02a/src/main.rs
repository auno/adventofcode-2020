use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut num_valid: usize = 0;
    let pattern = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>.): (?P<password>.*)$")
        .unwrap();

    for line in lines {
        if let Some(caps) = pattern.captures(line.as_str()) {
            let min: usize = caps.name("min").unwrap().as_str().parse().unwrap();
            let max: usize = caps.name("max").unwrap().as_str().parse().unwrap();
            let char: &str = caps.name("char").unwrap().as_str();
            let password: &str = caps.name("password").unwrap().as_str();

            let count = password.matches(char).count();
            if count >= min && count <= max {
                num_valid += 1;
                eprintln!("pass: {}", line);
            } else {
                eprintln!("fail: {}", line);
            }
        }
    }

    println!("{}", num_valid);
}
