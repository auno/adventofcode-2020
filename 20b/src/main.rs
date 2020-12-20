use std::io::{self, Read};
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use Direction::*;
use std::slice::Iter;
use std::cmp::{min, max};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn iter() -> Iter<'static, Direction> {
        lazy_static! {
            static ref DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];
        }

        DIRECTIONS.iter()
    }

    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn neighbor(&self, direction: &Direction) -> Position {
        match direction {
            Up => Position { x: self.x, y: self.y - 1 },
            Right => Position { x: self.x + 1, y: self.y },
            Down => Position { x: self.x, y: self.y + 1 },
            Left => Position { x: self.x - 1, y: self.y },
        }
    }
}

#[derive(Eq, Clone)]
struct Tile {
    id: i32,
    image: [[char; 10]; 10],
    borders: [u16; 8],
    rotation: u8,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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

        Ok(Tile { id, image, borders, rotation: 0 })
    }
}

impl Tile {
    fn border(&self, direction: &Direction) -> u16 {
        match self.rotation as usize {
            r if r < 4 => match direction {
                Up => self.borders[r],
                Right => self.borders[(r + 1) % 4],
                Down => self.borders[((r + 2) % 4) + 4],
                Left => self.borders[((r + 3) % 4) + 4],
            },
            r if r >= 4 && r < 8 => match direction {
                Up => self.borders[r],
                Right => self.borders[((r - 1) % 4) + 4],
                Down => self.borders[(r - 2) % 4],
                Left => self.borders[(r - 3) % 4],
            },
            r => panic!("Invalid rotation: {}", r)
        }
    }

    fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 8;
    }

    fn rotated_image(&self) -> Vec<Vec<char>> {
        let image: Vec<Vec<char>> = self.image.iter()
            .map(|line| line.iter().cloned().collect())
            .collect();

        rotate_image(&image, self.rotation)
    }
}

fn rotate_image(image: &Vec<Vec<char>>, rotation: u8) -> Vec<Vec<char>> {
    let x_max = image[0].len() - 1;
    let y_max = image.len() - 1;

    let mut rotated_image = vec![vec![' '; y_max + 1]; x_max + 1];

    for y in 0..=y_max {
        for x in 0..=x_max {
            let new_coords = match rotation {
                0 => (x, y),
                1 => (y, x_max - x),
                2 => (x_max - x, y_max - y),
                3 => (y_max - y, x),
                4 => (x_max - x, y),
                5 => (y_max - y, x_max - x),
                6 => (x, y_max - y),
                7 => (y, x),
                _ => panic!("Invalid rotation")
            };

            rotated_image[new_coords.1][new_coords.0] = image[y][x];
        }
    }

    rotated_image
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
    let mut tiles: HashMap<i32, Tile> = input.iter()
        .map(|s| s.parse::<Tile>().unwrap())
        .map(|tile| (tile.id, tile))
        .collect();
    let mut tiles_available: HashSet<i32> = tiles.values()
        .map(|tile| tile.id)
        .collect();
    let mut tiles_map: HashMap<Position, i32> = HashMap::new();
    let mut tiles_queue: VecDeque<(Position, i32)> = VecDeque::new();

    {
        let initial_tile = *tiles_available.iter().next().unwrap();
        tiles_available.remove(&initial_tile);
        tiles_map.insert(Position {x: 0, y: 0}, initial_tile);
        tiles_queue.push_back((Position {x: 0, y: 0}, initial_tile));
    }

    while let Some((position, tile_id)) = tiles_queue.pop_front() {
        for direction in Direction::iter() {
            let neighbor_position = position.neighbor(direction);

            if tiles_map.contains_key(&neighbor_position) {
                continue;
            }

            let border = tiles[&tile_id].border(&direction);

            let matching_tiles: Vec<i32> = tiles_available.iter()
                .filter(|candidate| tiles[&candidate].borders.contains(&border))
                .cloned()
                .collect();

            assert!(matching_tiles.len() <= 1);

            if let Some(neighbor_index) = matching_tiles.first() {
                let neighbor_tile = tiles.get_mut(&neighbor_index).unwrap();
                let neighbor_direction = direction.opposite();

                while border != neighbor_tile.border(&neighbor_direction) {
                    neighbor_tile.rotate();
                }

                tiles_available.remove(&neighbor_tile.id);
                tiles_map.insert(neighbor_position, neighbor_tile.id);
                tiles_queue.push_back((neighbor_position, neighbor_tile.id))
            }
        }
    }

    let (min, max) = tiles_map.keys()
        .fold((Position { x: 0, y: 0 }, Position { x: 0, y: 0 }), |acc, position| (
            Position { x: min(acc.0.x, position.x), y: min(acc.0.y, position.y) },
            Position { x: max(acc.1.x, position.x), y: max(acc.1.y, position.y) },
        ));

    let image: Vec<Vec<char>> = ((min.y)..=(max.y))
        .map(|y| {
            (1..9)
                .map(|tile_line| {
                    ((min.x)..=(max.x))
                        .map(|x| {
                            tiles[&tiles_map[&Position { x, y }]]
                                .rotated_image()[tile_line][1..9]
                                .iter()
                                .cloned()
                                .collect::<Vec<char>>()
                        })
                        .collect::<Vec<Vec<char>>>()
                        .concat()
                })
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
        .concat();

    let line_len = image[0].len();
    let monster_pattern = format!(
        "{}.{{{}}}{}.{{{}}}{}",
        "..................#.",
        line_len - 20,
        "#....##....##....###",
        line_len - 20,
        ".#..#..#..#..#..#...",
    );
    let monster_pattern: Regex = Regex::new(&monster_pattern).unwrap();

    for rotation in 0..8 {
        let image = rotate_image(&image, rotation);
        let oneline_image_full = image.iter()
            .map(|line| line.iter().cloned().collect::<String>())
            .collect::<String>();
        let mut oneline_image = oneline_image_full.as_str();

        let mut num_matches: usize = 0;

        while let Some(m) = monster_pattern.find(&oneline_image) {
            num_matches += 1;
            oneline_image = &oneline_image[(m.start() + 1)..];
        }

        if num_matches > 0 {
            let num_hash = oneline_image_full.chars().filter(|c| *c == '#').count();
            println!("{}", num_hash - num_matches * 15);
            break;
        }
    }
}
