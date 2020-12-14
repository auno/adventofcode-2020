use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut set_mask: u64 = 0;
    let mut reset_mask: u64 = 0;

    for line in input {
        let parts: Vec<&str> = line.split(" = ").collect();
        let command = parts[0];
        let value = parts[1];

        if command == "mask" {
            set_mask = u64::from_str_radix(&value.replace(|c| c != '1', "0"), 2).unwrap();
            reset_mask = u64::from_str_radix(&value.replace(|c| c != '0', "1"), 2).unwrap();
        } else {
            let address  = command.split(|c| c == '[' || c == ']').collect::<Vec<&str>>()[1];
            let address = address.parse::<usize>().unwrap();
            let value = value.parse::<u64>().unwrap() & reset_mask | set_mask;
            mem.insert(address, value);
        }
    }

    let sum: u64 = mem.values().sum();

    println!("{}", sum);
}