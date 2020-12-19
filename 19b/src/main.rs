use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Rule {
    Char(char),
    Reference(i32),
    List(Vec<Rule>),
    Disjunction(Vec<Rule>)
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(rule_string: &str) -> Result<Self, Self::Err> {
        if &rule_string[0..1] == "\"" {
            return Ok(Rule::Char(rule_string.chars().skip(1).next().unwrap()));
        } else if rule_string.contains(" | ") {
            let rules = rule_string
                .split(" | ")
                .map(|x| Rule::from_str(x).unwrap())
                .collect();
            return Ok(Rule::Disjunction(rules));
        } else if  rule_string.contains(" "){
            let rules = rule_string
                .split(" ")
                .map(|x| Rule::from_str(x).unwrap())
                .collect();
            return Ok(Rule::List(rules));
        } else {
            let number = rule_string.parse().unwrap();
            return Ok(Rule::Reference(number));
        }
    }
}

fn apply_rule(rules: &HashMap<i32, Rule>, rule: &Rule, candidate: &str) -> Option<usize> {
    if candidate.is_empty() {
        return None;
    }

    match rule {
        Rule::Char(c) => {
            match candidate.chars().next().unwrap() {
                next_char if next_char == *c => Some(1),
                _ => None
            }
        }
        Rule::Reference(rule_number) => {
            apply_rule(rules, &rules[rule_number], candidate)
        }
        Rule::List(list) => {
            list.iter().fold(Some(0), |acc, rule| {
                match acc {
                    None => None,
                    Some(already_parsed) => {
                        match apply_rule(rules, rule, &candidate[already_parsed..]) {
                            None => None,
                            Some(parsed) => Some(already_parsed + parsed)
                        }
                    }
                }
            })
        }
        Rule::Disjunction(disjunction) => {
            disjunction.iter().fold(None, |acc, rule| {
                match acc {
                    Some(parsed) => Some(parsed),
                    None => apply_rule(rules, rule, candidate)
                }
            })
        }
    }
}

fn is_parsable(rules: &HashMap<i32, Rule>, candidate: &str) -> bool {
    for num42 in 2..=(candidate.len() - 1) {
        for num31 in 1..=min(num42 - 1, candidate.len() - num42) {
            let rules42 = vec![rules[&42].clone(); num42];
            let rules31 = vec![rules[&31].clone(); num31];
            let combined = Rule::List([rules42, rules31].concat());

            if let Some(parsed) = apply_rule(rules, &combined, candidate) {
                if parsed == candidate.len() {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut input = input.iter();
    let mut rules: HashMap<i32, Rule> = HashMap::new();

    loop {
        let line = input.next().unwrap();

        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let rule_number: i32 = parts.next().unwrap().parse().unwrap();
        let rule: Rule = parts.next().unwrap().parse().unwrap();

        rules.insert(rule_number, rule);
    }

    let num_parsable = input
        .filter(|line| is_parsable(&rules, line.as_str()))
        .count();

    println!("{}", num_parsable);
}
