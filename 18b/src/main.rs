use std::io::{self, BufRead};
use regex::Regex;
use std::collections::VecDeque;
use crate::Token::*;
use crate::Op::*;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Op {
    Plus,
    Times
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Token {
    Operand(u64),
    Operator(Op),
    LeftParen,
    RightParen
}

fn tokenize(expression: &str) -> VecDeque<Token> {
    lazy_static! {
        static ref TOKEN_PATTERN: Regex = Regex::new(r"(\d+|[+*()])").unwrap();
    }

    TOKEN_PATTERN.find_iter(expression)
        .map(|m| {
            let token = &expression[m.start()..m.end()];
            match token {
                "+" => Operator(Op::Plus),
                "*" => Operator(Op::Times),
                "(" => LeftParen,
                ")" => RightParen,
                _ => Operand(token.parse().unwrap())
            }
        })
        .collect()
}

fn rpnize(expression: &VecDeque<Token>) -> VecDeque<Token> {
    let mut output: VecDeque<Token> = VecDeque::new();
    let mut operators: VecDeque<Token> = VecDeque::new();

    for token in expression {
        match token {
            Operand(operand) => {
                output.push_back(Operand(*operand));
            }
            Operator(operator) => {
                while let Some(token_on_stack) = operators.pop_front() {
                    match (operator, token_on_stack) {
                        (Times, Operator(Plus)) | (Times, Operator(Times)) | (Plus, Operator(Plus)) => {
                            output.push_back(token_on_stack);
                        }
                        _ => {
                            operators.push_front(token_on_stack);
                            break;
                        }
                    }
                }

                operators.push_front(Operator(*operator));
            }
            LeftParen => {
                operators.push_front(LeftParen);
            }
            RightParen => {
                while let Some(token_on_stack) = operators.pop_front() {
                    match token_on_stack {
                        Operator(op) => {
                            output.push_back(Operator(op));
                        }
                        LeftParen => {
                            break;
                        }
                        RightParen => {
                            panic!("Malformed expression")
                        }
                        Operand(_) => {
                            unreachable!();
                        }
                    }
                }
            }
        }
    }

    while let Some(token_on_stack) = operators.pop_front() {
        output.push_back(token_on_stack);
    }

    output
}

fn evaluate(rpn_expression: &VecDeque<Token>) -> u64 {
    let mut values: VecDeque<u64> = VecDeque::new();

    for token in rpn_expression {
        match token {
            Operand(value) => {
                values.push_front(*value);
            }
            Operator(operator) => {
                let v1 = values.pop_front().unwrap();
                let v2 = values.pop_front().unwrap();

                values.push_front(match operator {
                    Plus => v1 + v2,
                    Times => v1 * v2
                })
            }
            LeftParen | RightParen => {
                panic!("Malformed expression: {:?}", rpn_expression)
            }
        }
    }

    values.pop_front().expect(&format!("Malformed expression {:?}", rpn_expression))
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let sum: u64 = lines.iter()
        .map(|line| tokenize(&line))
        .map(|expression| rpnize(&expression))
        .map(|expression| evaluate(&expression))
        .sum();

    println!("{}", sum);
}
