use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut input = input.iter();
    let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();

    loop {
        let line = input.next().unwrap();

        if line.is_empty() {
            break;
        }

        let range_pair = line.split(": ").skip(1).next().unwrap();
        range_pair.split(" or ")
            .map(|range| {
                let mut range_parts = range.split("-");
                let start: i32 = range_parts.next().unwrap().parse().unwrap();
                let end: i32 = range_parts.next().unwrap().parse().unwrap();
                start..=end
            })
            .for_each(|range| ranges.push(range))
    }

    let error_rate: i32 = input.skip(4)
        .flat_map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()))
        .filter(|num| !ranges.iter().any(|range| range.contains(num)))
        .sum();

    println!("{}", error_rate);
}