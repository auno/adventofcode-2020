use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let mut parts = line.split(" (");

    let ingredients = parts.next().unwrap();
    let allergens = &parts.next()
        .unwrap()
        .strip_prefix("contains ")
        .unwrap()
        .strip_suffix(")")
        .unwrap();

    let ingredients = ingredients
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    let allergens = allergens
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    (ingredients, allergens)
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut partial_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let (ingredients, allergens) = parse_line(&line);


        for allergen in allergens {
            if !partial_mapping.contains_key(&allergen) {
                partial_mapping.insert(allergen.clone(), ingredients.iter().cloned().collect());
                continue;
            }

            partial_mapping
                .get_mut(&allergen)
                .unwrap()
                .retain(|ingredient| ingredients.contains(ingredient));
        }
    }

    let mut final_mapping: HashMap<String, String> = HashMap::new();

    while final_mapping.len() < partial_mapping.len() {
        let (determined_allergen, determined_ingredient) = partial_mapping.iter()
            .filter(|(allergen, _ingredients)| !final_mapping.contains_key(*allergen))
            .filter(|(_allergen, ingredients)| ingredients.len() == 1)
            .map(|(allergen, _ingredients)| (allergen.clone(), _ingredients.iter().next().unwrap().clone()))
            .next()
            .unwrap();

        partial_mapping.iter_mut()
            .filter(|(allergen, _ingredients)| allergen != &&determined_allergen)
            .for_each(|(_allergen, ingredients)| {
                ingredients.remove(&determined_ingredient);
            });

        final_mapping.insert(determined_allergen, determined_ingredient);
    }


    let mut list: Vec<(&String, &String)> = final_mapping.iter().collect();
    list.sort_by_key(|(allergen, _ingredient)| allergen.clone());
    let list = &list.iter()
        .map(|(_allergen, ingredient)| *ingredient)
        .cloned()
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", list);
}
