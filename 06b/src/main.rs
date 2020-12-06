use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let groups: Vec<&[String]> = lines.split(|line| line.trim().is_empty()).collect();

    let sum: usize = groups.iter()
        .map(|group| {
            group.iter()
                .map(|member| member.chars().collect::<HashSet<_>>())
                .fold(('a'..='z').collect::<HashSet<_>>(), |acc, x| {
                    acc.intersection(&x).cloned().collect::<HashSet<_>>()
                })
                .len()
        })
        .sum();

    println!("{}", sum);
}
