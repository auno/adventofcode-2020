use std::io::{self, BufRead};
use crate::Instruction::{North, South, East, West, Turn};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Turn(i32)
}

impl From<(char, &str)> for Instruction {
    fn from(v: (char, &str)) -> Self {
        let (current_direction, instruction) = v;
        match (&instruction[0..1], instruction[1..].parse::<i32>().unwrap()) {
            ("N", amount) => North(amount),
            ("S", amount) => South(amount),
            ("E", amount) => East(amount),
            ("W", amount) => West(amount),
            ("R", amount) => Turn(amount),
            ("L", amount) => Turn(-amount),
            ("F", amount) => Instruction::from((current_direction, format!("{}{}", current_direction, amount).as_str())),
            _ => panic!("Unknown instruction: {}", instruction)
        }
    }
}

fn turn(direction: char, amount: i32) -> char {
    let mut direction = match direction {
        'N' => 0,
        'E' => 90,
        'S' => 180,
        'W' => 270,
        _ => panic!("Unknown direction: {}", direction)
    };

    direction += amount;

    match direction.rem_euclid(360) {
        0 => 'N',
        90 => 'E',
        180 => 'S',
        270 => 'W',
        _ => panic!("Unknown direction: {}", direction)
    }
}

fn main() {
    let instructions: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let (_, position) = instructions.iter().fold(('E', (0, 0)), |acc, instruction| {
        let (direction, position) = acc;
        let instruction = Instruction::from((direction, instruction.as_str()));

        match instruction {
            North(amount) => (direction, (position.0, position.1 + amount)),
            South(amount) => (direction, (position.0, position.1 - amount)),
            East(amount) => (direction, (position.0 + amount, position.1)),
            West(amount) => (direction, (position.0 - amount, position.1)),
            Turn(amount) => (turn(direction, amount), position)
        }
    });

    println!("{}", position.0.abs() + position.1.abs());
}