use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, HashSet};
use regex::Regex;

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let outer = Regex::new(r"^(?P<color>.*) bags contain").unwrap();
    let inner = Regex::new(r"(?P<count>\d+) (?P<color>[^,]*) bag").unwrap();

    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let caps = outer.captures(&line).unwrap();
        let outer_color = String::from(caps.name("color").unwrap().as_str());

        inner.captures_iter(&line).for_each(|caps| {
            let inner_color = String::from(caps.name("color").unwrap().as_str());

            if !parents.contains_key(&inner_color) {
                parents.insert(inner_color.clone(), Vec::new());
            }

            let pv = parents.get_mut(&inner_color).unwrap();
            pv.push(String::from(outer_color.clone()));
        })
    }

    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("shiny gold");
    let mut bags: HashSet<&str> = HashSet::new();

    while !queue.is_empty() {
        let current_color = queue.pop_front().unwrap();
        bags.insert(current_color);

        if parents.contains_key(current_color) {
            parents
                .get(current_color)
                .unwrap()
                .iter()
                .for_each(|parent| queue.push_back(parent));
        }
    }

    println!("{}", bags.len() - 1);
}
