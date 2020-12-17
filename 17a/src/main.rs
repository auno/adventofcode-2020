use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize
}

impl Coordinate {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Coordinate { x, y, z }
    }

    fn adjacent(&self) -> HashSet<Coordinate> {
        let mut adjacent: HashSet<Coordinate> = HashSet::new();
        for x in self.x-1..=self.x+1 {
            for y in self.y-1..=self.y+1 {
                for z in self.z-1..=self.z+1 {
                    adjacent.insert(Coordinate::new(x, y, z));
                }
            }
        }

        adjacent
    }
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut active: HashSet<Coordinate> = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let coordinate = Coordinate::new(x as isize, y as isize, 0);
                active.insert(coordinate);
            }
        }
    }

    for _turn in 0..6 {
        let active_adjacent: HashSet<Coordinate> = active
            .iter()
            .flat_map(|coordinate| coordinate.adjacent())
            .collect();
        let mut next_active: HashSet<Coordinate> = HashSet::new();

        for coordinate in &active_adjacent {
            let adjacent = coordinate.adjacent();
            let num_active_adjacent = adjacent
                .iter()
                .filter(|c| c != &coordinate)
                .filter(|c| active.contains(c))
                .count();
            match (active.contains(coordinate), num_active_adjacent) {
                (true, 2) | (true, 3) | (false, 3) => { next_active.insert(*coordinate); },
                _ => {}
            }
        }

        active = next_active;
    }

    println!("{}", active.len());
}