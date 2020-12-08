use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
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
                    "nop" => Ok(Instruction::Nop(operand)),
                    _ => Err(InstructionParseError::UnknownInstruction)
                }
            }
        }

        return Err(InstructionParseError::InvalidFormat);
    }
}

#[derive(Debug)]
enum SimulationError {
    InfiniteLoop(usize, i32),
    Crash(usize, i32)
}

fn simulate(instructions: &Vec<Instruction>) -> Result<i32, SimulationError> {
    let mut visited: Vec<bool> = instructions.iter().map(|_| false).collect();
    let mut pc: usize = 0;
    let mut acc: i32 = 0;

    loop {
        if pc == instructions.len() { return Ok(acc); }
        if visited[pc] { return Err(SimulationError::InfiniteLoop(pc, acc)); }

        visited[pc] = true;

        match instructions[pc] {
            Instruction::Acc(operand) => {
                acc += operand;
                pc += 1;
            }
            Instruction::Jmp(operand) => {
                let new_pc = pc as i32 + operand;

                if new_pc < 0 || new_pc as usize > instructions.len() {
                    return Err(SimulationError::Crash(pc, acc))
                }

                pc = new_pc as usize;
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    for i in 0..instructions.len() {
        let mut instructions_clone = instructions.clone();

        match instructions_clone[i] {
            Instruction::Acc(_) => { continue; }
            Instruction::Jmp(operand) => { instructions_clone[i] = Instruction::Nop(operand); }
            Instruction::Nop(operand) => { instructions_clone[i] = Instruction::Jmp(operand); }
        }

        if let Ok(acc) = simulate(&instructions_clone) {
            println!("{}", acc);
            break;
        }
    }
}
