use std::io::{self, BufRead};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Tile {
    e: isize,
    se: isize,
}

fn next(path: &str) -> &str {
    match &path[0..1] {
        "e" | "w" => &path[0..1],
        _ => &path[0..2]
    }
}

impl Tile {
    fn new(e: isize, se: isize) -> Self {
        Tile { e, se }
    }

    fn adjacent(&self) -> HashSet<Tile> {
        [
            self.clone(),
            Tile::new(self.e - 1, self.se),
            Tile::new(self.e + 1, self.se),
            Tile::new(self.e, self.se - 1),
            Tile::new(self.e, self.se + 1),
            Tile::new(self.e - 1, self.se + 1),
            Tile::new(self.e + 1, self.se - 1),
        ].iter().cloned().collect()
    }
}

#[derive(Debug)]
enum ParseTileError {
    InvalidFormat
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path = s;
        let mut tile = Tile::new(0, 0);

        while path.len() > 0 {
            match next(path) {
                "e" => {
                    tile.e += 1;
                    path = &path[1..];
                }
                "w" => {
                    tile.e -= 1;
                    path = &path[1..];
                }
                "se" => {
                    tile.se += 1;
                    path = &path[2..];
                }
                "nw" => {
                    tile.se -= 1;
                    path = &path[2..];
                }
                "ne" => {
                    tile.e += 1;
                    tile.se -= 1;
                    path = &path[2..];
                }
                "sw" => {
                    tile.e -= 1;
                    tile.se += 1;
                    path = &path[2..];
                }
                _ => {
                    return Err(ParseTileError::InvalidFormat)
                }
            }
        }

        Ok(tile)
    }
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut active: HashSet<Tile> = HashSet::new();

    for line in &input {
        let tile: Tile = line.parse().unwrap();

        match active.contains(&tile) {
            true => { active.remove(&tile); }
            false => { active.insert(tile); }
        }
    }

    for _turn in 0..100 {
        let active_adjacent: HashSet<Tile> = active
            .iter()
            .flat_map(|coordinate| coordinate.adjacent())
            .collect();
        let mut next_active: HashSet<Tile> = HashSet::new();

        for coordinate in &active_adjacent {
            let adjacent = coordinate.adjacent();
            let num_active_adjacent = adjacent
                .iter()
                .filter(|c| c != &coordinate)
                .filter(|c| active.contains(c))
                .count();
            match (active.contains(coordinate), num_active_adjacent) {
                (true, 1) | (_, 2) => { next_active.insert(*coordinate); },
                _ => {}
            }
        }

        active = next_active;
    }

    println!("{}", active.len());
}
