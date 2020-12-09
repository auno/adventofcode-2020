use std::io::{self, BufRead};

fn find_weakness(numbers: &[i64], preamble_size: usize) -> i64 {
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

        return numbers[i];
    }

    panic!("Failed to find weakness");
}

fn main() {
    let numbers: Vec<i64> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    let preamble_size = 25;
    let weakness = find_weakness(&numbers, preamble_size);

    'outer: for i in 0..(numbers.len() - 1) {
        let mut sum = numbers[i];

        for j in (i + 1)..numbers.len() {
            sum += numbers[j];

            if sum == weakness {
                let mut range = Vec::from(&numbers[i..=j]);
                range.sort();
                println!("{}", range.first().unwrap() + range.last().unwrap());
                break 'outer;
            }

            if sum > weakness {
                continue 'outer;
            }
        }
    }
}
