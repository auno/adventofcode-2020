use std::io::{self, BufRead};

fn main() {
    let numbers: Vec<i64> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    let preamble_size = 25;

    'outer: for i in preamble_size..numbers.len() {
        for j in (i - preamble_size)..(i - 1) {
            for k in (j + 1)..i {
                if numbers[j] == numbers[k] {
                    continue;
                }

                if numbers[j] + numbers[k] == numbers[i] {
                    continue 'outer;
                }
            }
        }

        println!("{}", numbers[i]);
    }
}
