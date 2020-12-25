use std::io::{self, BufRead};

fn calculate_loop_sizes(public_keys: &Vec<u64>) -> Vec<u64> {
    let mut loop_sizes = vec![None; public_keys.len()];
    let mut size = 0;
    let mut current = 1;

    while loop_sizes.iter().any(|ls| ls.is_none()) {
        for i in 0..public_keys.len() {
            if current == public_keys[i] {
                loop_sizes[i] = Some(size);
            }
        }

        size += 1;
        current = (current * 7) % 20201227;
    }

    loop_sizes.iter()
        .map(|ls| ls.unwrap())
        .collect()
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |acc, _| (acc * subject) % 20201227)
}

fn main() {
    let public_keys: Vec<u64> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    let loop_sizes = calculate_loop_sizes(&public_keys);
    eprintln!("Loop Sizes: {:?}", loop_sizes);

    let encryption_key = transform(public_keys[0], loop_sizes[1]);
    println!("{}", encryption_key);
}
