use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop
}

#[derive(Debug)]
enum InstructionParseError {
    InvalidFormat,
    UnknownInstruction
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() == 2 {
            if let Ok(operand) = parts[1].parse::<i32>() {
                return match parts[0] {
                    "acc" => Ok(Instruction::Acc(operand)),
                    "jmp" => Ok(Instruction::Jmp(operand)),
                    "nop" => Ok(Instruction::Nop),
                    _ => Err(InstructionParseError::UnknownInstruction)
                }
            }
        }

        return Err(InstructionParseError::InvalidFormat);
    }
}

fn main() {
    let instructions: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    let mut visited: Vec<bool> = instructions.iter().map(|_| false).collect();
    let mut pc: usize = 0;
    let mut acc: i32 = 0;

    while !visited[pc] {
        visited[pc] = true;

        match instructions[pc] {
            Instruction::Acc(operand) => {
                acc += operand;
                pc += 1;
            }
            Instruction::Jmp(operand) => {
                pc = (pc as i32 + operand) as usize;
            }
            Instruction::Nop => {
                pc += 1;
            }
        }
    }

    println!("{}", acc);
}
