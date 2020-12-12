use std::io::{self, BufRead};
use crate::Instruction::{North, South, East, West, Rotate, Forward};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Rotate(i32),
    Forward(i32)
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        match (&instruction[0..1], instruction[1..].parse::<i32>().unwrap()) {
            ("N", amount) => North(amount),
            ("S", amount) => South(amount),
            ("E", amount) => East(amount),
            ("W", amount) => West(amount),
            ("R", amount) => Rotate(amount),
            ("L", amount) => Rotate(-amount),
            ("F", amount) => Forward(amount),
            _ => panic!("Unknown instruction: {}", instruction)
        }
    }
}

fn rotate(waypoint: (i32, i32), amount: i32) -> (i32, i32) {
    match amount.rem_euclid(360) {
        0 => waypoint,
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => panic!("Unknown rotation amount: {}", amount)
    }
}

fn main() {
    let instructions: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let (_, position) = instructions.iter().fold(((10, 1), (0, 0)), |acc, instruction| {
        let (waypoint, position) = acc;
        let instruction = Instruction::from(instruction.as_str());

        match instruction {
            North(amount) => ((waypoint.0, waypoint.1 + amount), position),
            South(amount) => ((waypoint.0, waypoint.1 - amount), position),
            East(amount) => ((waypoint.0 + amount, waypoint.1), position),
            West(amount) => ((waypoint.0 - amount, waypoint.1), position),
            Rotate(amount) => (rotate(waypoint, amount), position),
            Forward(amount) => (waypoint, (position.0 + waypoint.0 * amount, position.1 + waypoint.1 * amount))
        }
    });

    println!("{}", position.0.abs() + position.1.abs());
}