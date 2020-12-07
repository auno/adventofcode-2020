use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn calc(children: &HashMap<String, Vec<(String, usize)>>, current: &str) -> usize {
    let sum_children: usize = children
        .get(current)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|(color, count)| count * calc(children, color))
        .sum();

    sum_children + 1
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let outer = Regex::new(r"^(?P<color>.*) bags contain").unwrap();
    let inner = Regex::new(r"(?P<count>\d+) (?P<color>[^,]*) bag").unwrap();

    let mut children: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for line in lines {
        let caps = outer.captures(&line).unwrap();
        let outer_color = String::from(caps.name("color").unwrap().as_str());

        inner.captures_iter(&line).for_each(|caps| {
            let inner_color = String::from(caps.name("color").unwrap().as_str());
            let inner_count: usize = caps.name("count").unwrap().as_str().parse().unwrap();

            if !children.contains_key(&outer_color) {
                children.insert(outer_color.clone(), Vec::new());
            }

            let cv = children.get_mut(&outer_color).unwrap();
            cv.push((String::from(inner_color.clone()), inner_count));
        })
    }

    println!("{}", calc(&children, "shiny gold") - 1);
}