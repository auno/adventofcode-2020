use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut num_valid: usize = 0;
    let pattern = Regex::new(r"^(?P<pos1>\d+)-(?P<pos2>\d+) (?P<char>.): (?P<password>.*)$")
        .unwrap();

    for line in lines {
        if let Some(caps) = pattern.captures(line.as_str()) {
            let pos1: usize = caps.name("pos1").unwrap().as_str().parse().unwrap();
            let pos2: usize = caps.name("pos2").unwrap().as_str().parse().unwrap();
            let char: &str = caps.name("char").unwrap().as_str();
            let password: &str = caps.name("password").unwrap().as_str();

            let pos1_matches = &password[(pos1 - 1)..pos1] == char;
            let pos2_matches = &password[(pos2 - 1)..pos2] == char;

            if (pos1_matches || pos2_matches) && !(pos1_matches && pos2_matches) {
                num_valid += 1;
                eprintln!("pass: {}", line);
            } else {
                eprintln!("fail: {}", line);
            }
        }
    }

    println!("{}", num_valid);
}
