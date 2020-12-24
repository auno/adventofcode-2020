use std::io::{self, BufRead};
use std::collections::HashSet;

fn next(path: &str) -> &str {
    match &path[0..1] {
        "e" | "w" => &path[0..1],
        _ => &path[0..2]
    }
}

fn parse_tile_path(path: &str) -> (i32, i32) {
    let mut path = path;
    let mut tile = (0, 0);

    while path.len() > 0 {
        match next(path) {
            "e" => {
                tile.0 += 1;
                path = &path[1..];
            }
            "w" => {
                tile.0 -= 1;
                path = &path[1..];
            }
            "se" => {
                tile.1 += 1;
                path = &path[2..];
            }
            "nw" => {
                tile.1 -= 1;
                path = &path[2..];
            }
            "ne" => {
                tile.0 += 1;
                tile.1 -= 1;
                path = &path[2..];
            }
            "sw" => {
                tile.0 -= 1;
                tile.1 += 1;
                path = &path[2..];
            }
            unknown => {
                panic!("Unknown path element: {}", unknown);
            }
        }
    }

    tile
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();

    for line in input {
        let tile = parse_tile_path(&line);

        match black_tiles.contains(&tile) {
            true => { black_tiles.remove(&tile); }
            false => { black_tiles.insert(tile); }
        }
    }

    println!("{}", black_tiles.len());
}
