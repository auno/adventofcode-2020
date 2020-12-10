use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    let mut adapters: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    adapters.push(0);
    let device = adapters.iter().max().unwrap() + 3;
    adapters.push(device);
    adapters.sort();
    let adapters = adapters;

    let mut num_paths: HashMap<i32, usize> = HashMap::new();
    num_paths.insert(device, 1);

    (0..(adapters.len() - 1))
        .rev()
        .for_each(|i| {
            let current = adapters[i];
            let reachable = adapters[(i + 1)..=min(i + 3, adapters.len() - 1)]
                .iter()
                .filter(|n| **n - current <= 3);
            let current_num_paths: usize = reachable
                .map(|n| num_paths[n])
                .sum();

            num_paths.insert(current, current_num_paths);
        });

    println!("{}", num_paths[&0]);
}
