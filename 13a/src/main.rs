use std::io::{self, BufRead};

fn calc_wait_time(lower_bound: i32, bus: i32) -> i32 {
    bus - (lower_bound % bus)
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let lower_bound: i32 = input[0].parse().unwrap();
    let next_bus = input[1].split(',')
        .filter(|candidate| candidate != &"x")
        .map(|bus| bus.parse::<i32>().unwrap())
        .min_by(|a, b| calc_wait_time(lower_bound, *a).cmp(&calc_wait_time(lower_bound, *b)))
        .unwrap();

    let wait_time = calc_wait_time(lower_bound, next_bus);

    println!("{}", wait_time * next_bus);
}