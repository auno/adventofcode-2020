use std::io::{self, BufRead};
use std::collections::HashMap;

use Tile::*;
use std::cmp::max;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Open,
            '#' => Tree,
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Open => '.',
            Tree => '#',
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

fn read_map() -> (HashMap<Position, Tile>, i32) {
    let mut position: Position = (0, 0).into();
    let mut map: HashMap<Position, Tile> = HashMap::new();
    let mut width: i32 = 0;

    let input: String = io::stdin().lock().lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");

    for c in input.chars() {
        match c {
            '\n' => {
                position = (0, position.y + 1).into();
            },
            _ => {
                map.insert(position, c.into());
                position = (position.x + 1, position.y).into();
                width = max(width, position.x);
            }
        }
    }

    (map, width)
}

fn main() {
    let (map, width) = read_map();
    let mut position: Position = (0, 0).into();
    let mut num_trees: usize = 0;

    while map.contains_key(&position) {
        match map.get(&position) {
            None => { break; }
            Some(Open) => {}
            Some(Tree) => { num_trees += 1; }
        }

        position = ((position.x + 3) % width, position.y + 1).into();
    }

    println!("{}", num_trees);
}
