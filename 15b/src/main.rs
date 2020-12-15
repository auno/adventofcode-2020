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

    let mut previous_nums: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut turn: u32 = 0;

    for num in &input {
        previous_nums.entry(*num).or_insert(Vec::new()).push(turn);
        turn += 1;
    }

    let mut last_num: u32 = *input.last().unwrap();

    while turn < 30000000 {
        let last_num_history= previous_nums.get_mut(&last_num).unwrap();
        if last_num_history.len() > 1 {
            let a = last_num_history[last_num_history.len() - 2];
            let b = last_num_history[last_num_history.len() - 1];

            last_num = b - a;
            previous_nums.entry(last_num).or_insert(Vec::new()).push(turn);
        } else {
            last_num = 0;
            previous_nums.entry(last_num).or_insert(Vec::new()).push(turn);
        }

        turn += 1;
    }

    println!("{}", last_num);
}