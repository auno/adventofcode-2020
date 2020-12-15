use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let input: Vec<u32> = input[0]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut previous_nums: HashMap<u32, u32> = HashMap::new();
    let mut turn: u32 = 1;

    for num in &input[0..(input.len() - 1)] {
        previous_nums.insert(*num, turn);
        turn += 1;
    }

    let mut last_num: u32 = *input.last().unwrap();

    while turn < 2020 {
        if let Some(last_num_turn) = previous_nums.get(&last_num).cloned() {
            previous_nums.insert(last_num, turn);
            last_num = (turn) - last_num_turn;
        } else {
            previous_nums.insert(last_num, turn);
            last_num = 0;
        }

        turn += 1;
    }

    println!("{}", last_num);
}