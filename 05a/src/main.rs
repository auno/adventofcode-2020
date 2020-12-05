use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut max_seat: u32 = 0;

    for line in lines {
        let line = line
            .replace(|c| c == 'F' || c == 'L', "0")
            .replace(|c| c == 'B' || c == 'R', "1");

        let seat = u32::from_str_radix(&line, 2).unwrap();
        max_seat = max(max_seat, seat);
    }

    println!("{}", max_seat);
}
