use std::io::{self, BufRead};

fn main() {
    let mut adapters: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    adapters.push(0);
    let device = adapters.iter().max().unwrap() + 3;
    adapters.push(device);
    adapters.sort();

    let mut diff1 = 0;
    let mut diff3 = 0;

    adapters.windows(2)
        .for_each(|w| match w[1] - w[0] {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        });

    println!("{}", diff1 * diff3);
}
