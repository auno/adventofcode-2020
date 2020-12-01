use std::io::{self, BufRead};

fn find_sum(nums: &[i32], sum: i32) -> i32 {
    for a in nums.iter() {
        for b in nums.iter().rev() {
            if a + b == sum {
                return a * b;
            }

            if a + b < sum {
                break;
            }
        }
    }

    return -1;
}

fn main() {
    let mut nums: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    nums.sort();
    let nums = nums;

    println!("{}", find_sum(&nums, 2020));
}
