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

    let mut potential_mapping: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients_list: Vec<String> = Vec::new();
    let mut all_ingredients_set: HashSet<String> = HashSet::new();

    for line in input {
        let (ingredients, allergens) = parse_line(&line);

        ingredients.iter().for_each(|ingredient| {
            all_ingredients_list.push(ingredient.clone());
            all_ingredients_set.insert(ingredient.clone());
        });

        for allergen in allergens {
            if !potential_mapping.contains_key(&allergen) {
                potential_mapping.insert(allergen.clone(), ingredients.iter().cloned().collect());
                continue;
            }

            potential_mapping
                .get_mut(&allergen)
                .unwrap()
                .retain(|ingredient| ingredients.contains(ingredient));
        }
    }

    let potential_bad: HashSet<String> = potential_mapping.iter()
        .flat_map(|(_allergen, ingredients)| ingredients.iter())
        .cloned()
        .collect();

    let safe_ingredients: HashSet<String> = all_ingredients_set.difference(&potential_bad).cloned().collect();
    let num_safe_ingredients = all_ingredients_list.iter()
        .filter(|ingredient| safe_ingredients.contains(*ingredient))
        .count();

    println!("{}", num_safe_ingredients);
}
