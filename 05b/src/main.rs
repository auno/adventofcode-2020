use std::io::{self, BufRead};

fn parse_seat_number(seat_id: &str) -> u32 {
    let seat_id = seat_id
        .replace(|c| c == 'F' || c == 'L', "0")
        .replace(|c| c == 'B' || c == 'R', "1");

    u32::from_str_radix(&seat_id, 2).unwrap()
}

fn read_seats() -> Vec<u32> {
    let mut seats: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| parse_seat_number(&line))
        .collect();
    seats.sort();

    seats
}

fn main() {
    let seats = read_seats();

    let my_seat = seats
        .windows(2)
        .find_map(|w| {
            if w[1] == w[0] + 2 {
                Some(w[0] + 1)
            } else {
                None
            }
        });

    println!("{}", my_seat.unwrap());
}
