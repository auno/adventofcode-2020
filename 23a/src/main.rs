use std::io::{self, BufRead};

fn next_three(cups: &Vec<usize>, current: usize) -> Vec<usize> {
    let mut next_tree = vec![];
    let mut current = current;

    for _ in 0..3 {
        current = cups[current];
        next_tree.push(current);
    }

    next_tree
}

fn main() {
    let input: String = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .next()
        .unwrap();

    let cups_list: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let min = *cups_list.iter().min().unwrap();
    let max = *cups_list.iter().max().unwrap();

    let mut cups: Vec<usize> = vec![0; max + min];
    cups[*cups_list.last().unwrap()] = *cups_list.first().unwrap();

    cups_list.windows(2)
        .for_each(|w| {
            cups[w[0]] = w[1];
        });

    let mut current = cups_list[0];

    for _round in 0..100 {
        let next_three = next_three(&mut cups, current);

        let mut target = current;
        loop {
            target = ((target as i64 - min as i64 - 1).rem_euclid(max as i64 + 1 - min as i64) + min as i64) as usize;

            if !next_three.contains(&target) {
                break;
            }
        }

        cups[current] = cups[next_three[2]];
        let tmp = cups[target];
        cups[target] = next_three[0];
        cups[next_three[2]] = tmp;

        current = cups[current];
    }

    let mut current = cups[1];

    while current != 1 {
        print!("{}", current);
        current = cups[current];
    }

    println!();
}
