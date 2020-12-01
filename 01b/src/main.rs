use std::io::{self, BufRead};

fn find_product_matching_sum(nums: &[i32], target: i32) -> Option<i32> {
    for a in nums.iter() {
        for b in nums.iter() {
            if a + b > target {
                continue;
            }

            for c in nums.iter().rev() {
                if a + b + c == target {
                    return Some(a * b * c);
                }

                if a + b + c < target {
                    break;
                }
            }
        }
    }

    None
}

fn main() {
    let mut nums: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    nums.sort();
    let nums = nums;
    let product = find_product_matching_sum(&nums, 2020).unwrap();
    println!("{}", product);
}
