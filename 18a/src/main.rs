use std::io::{self, BufRead};
use regex::Regex;
use crate::Token::{Operand, Operator, LeftParen, RightParen};
use crate::Op::{Plus, Times};
use std::collections::VecDeque;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Plus,
    Times
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Operand(u64),
    Operator(Op),
    LeftParen,
    RightParen
}

fn evaluate(expression: &mut VecDeque<Token>) -> u64 {
    let mut value = 0;
    let mut operator = Plus;

    while !expression.is_empty() {
        match expression.pop_front() {
            Some(LeftParen) => {
                let subexpression_value = evaluate(expression);
                expression.push_front(Operand(subexpression_value));
            },
            Some(RightParen) | None => {
                return value;
            },
            Some(Operator(current)) => {
                operator = current;
            }
            Some(Operand(current)) => {
                value = match operator {
                    Plus => value + current,
                    Times => value * current,
                }
            }
        }
    }

    value
}

fn tokenize(expression: &str) -> VecDeque<Token> {
    lazy_static! {
        static ref TOKEN_PATTERN: Regex = Regex::new(r"(\d+|[+*()])").unwrap();
    }

    TOKEN_PATTERN.find_iter(expression)
        .map(|m| {
            let token = &expression[m.start()..m.end()];
            match token {
                "+" => Operator(Plus),
                "*" => Operator(Times),
                "(" => LeftParen,
                ")" => RightParen,
                _ => Operand(token.parse().unwrap())
            }
        })
        .collect()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut expressions: Vec<VecDeque<Token>> = lines.iter()
        .map(|line| tokenize(&line))
        .collect();

    let sum: u64 = expressions.iter_mut()
        .map(|expression| evaluate(expression))
        .sum();

    println!("{}", sum);
}
