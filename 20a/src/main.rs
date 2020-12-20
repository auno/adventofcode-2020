use std::io::{self, Read};
use std::str::FromStr;
use std::convert::TryInto;

#[derive(Eq)]
struct Tile {
    id: i32,
    borders: [u16; 8]
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..].trim_end_matches(":").parse().unwrap();
        let image: [[char; 10]; 10] = lines
            .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
            .collect::<Vec<[char; 10]>>()
            .try_into()
            .unwrap();
        let borders: Vec<Vec<char>> = (0..10)
            .fold(vec![vec![]; 4], |mut acc, i| {
                acc[0].push(image[0][i]);
                acc[1].push(image[i][9]);
                acc[2].push(image[9][9 - i]);
                acc[3].push(image[9 - i][0]);
                acc
            });
        let borders: Vec<Vec<char>> = [
            borders.clone(),
            borders.iter()
                .map(|border| border.iter()
                    .rev()
                    .cloned()
                    .collect()
                )
                .collect()
        ].concat();
        let borders: [u16; 8] = borders.iter()
            .map(|border| {
                border
                    .as_slice()
                    .iter()
                    .fold(0, |acc, c| {
                        (acc << 1) + match c {
                            '.' => 0,
                            '#' => 1,
                            _ => panic!("Unknown tile character: {}", c)
                        }
                    })
            })
            .collect::<Vec<u16>>()
            .try_into()
            .unwrap();

        Ok(Tile { id, borders })
    }
}

fn read_input() -> Vec<String> {
    let mut input: String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let input = read_input();
    let tiles: Vec<Tile> = input.iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let corner_product: i64 = tiles.iter()
        .filter(|candidate| {
            tiles.iter()
                .filter(|other| other != candidate)
                .filter(|other| candidate.borders.iter().any(|border| other.borders.contains(border)))
                .count() == 2
        })
        .fold(1, |acc, tile| acc * tile.id as i64);

    println!("{}", corner_product);
}
