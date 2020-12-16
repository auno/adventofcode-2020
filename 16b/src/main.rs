use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct NamedDisjointRange {
    name: String,
    range1: RangeInclusive<i64>,
    range2: RangeInclusive<i64>
}

impl FromStr for NamedDisjointRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut kv = s.split(": ");
        let name = kv.next().unwrap().to_string();

        let mut range_pair = kv.next().unwrap().split(" or ")
            .map(|range| {
                let mut range_parts = range.split("-");
                let start: i64 = range_parts.next().unwrap().parse().unwrap();
                let end: i64 = range_parts.next().unwrap().parse().unwrap();
                start..=end
            });

        let range1 = range_pair.next().unwrap();
        let range2 = range_pair.next().unwrap();

        Ok(NamedDisjointRange{name, range1, range2})
    }
}

impl NamedDisjointRange {
    fn contains(&self, value: &i64) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }
}

fn parse_ticket(ticket: &str) -> Vec<i64> {
    ticket
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut input = input.iter();
    let mut ranges: Vec<NamedDisjointRange> = Vec::new();

    loop {
        let line = input.next().unwrap();

        if line.is_empty() {
            break;
        }

        ranges.push(line.parse().unwrap())
    }

    let tickets: Vec<Vec<i64>> = input
        .filter(|line| !line.is_empty())
        .filter(|line| !['y', 'n'].contains(&line.chars().next().unwrap()))
        .map(|line| parse_ticket(line))
        .filter(|ticket| ticket.iter().all(|num| ranges.iter().any(|range| range.contains(num))))
        .collect();
    let my_ticket = &tickets[0];

    let mut candidates: Vec<Vec<&NamedDisjointRange>> = (0..ranges.len())
        .map(|i| {
            ranges.iter()
                .filter(|range| tickets
                    .iter()
                    .all(|ticket| range.contains(&ticket[i]))
                )
                .collect()
        })
        .collect();

    let mut done: HashSet<usize> = HashSet::new();

    while done.len() < candidates.len() {
        for i in 0..candidates.len() {
            if !done.contains(&i) && candidates[i].len() == 1 {
                done.insert(i);

                for j in 0..candidates.len() {
                    if j == i {
                        continue;
                    }

                    candidates[j] = candidates[j].iter()
                        .filter(|range| **range != candidates[i][0])
                        .cloned()
                        .collect();
                }
            }
        }
    }

    let ordering: Vec<&NamedDisjointRange> = candidates.iter()
        .map(|c| *c.first().unwrap())
        .collect();

    let departure_product = ordering
        .iter()
        .enumerate()
        .filter(|(_, range)| range.name.starts_with("departure "))
        .map(|(i, _)| my_ticket[i])
        .fold(1, |acc, value| acc * value);

    println!("{}", departure_product);
}