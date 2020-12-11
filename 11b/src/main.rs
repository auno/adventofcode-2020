use std::{io, fmt};
use std::io::BufRead;
use std::collections::HashMap;
use std::fmt::Display;

use Tile::*;
use std::cmp::max;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Occupied,
    Empty,
    Floor
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Occupied,
            'L' => Empty,
            '.' => Floor,
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Occupied => '#',
            Empty => 'L',
            Floor => '.',
        };
        write!(f, "{}", c)
    }
}

type Game = HashMap<(isize, isize), Tile>;

fn get_seat_by_direction(game: &Game, x: isize, y: isize, x_diff: isize, y_diff: isize) -> Option<&Tile> {
    let x = x + x_diff;
    let y = y + y_diff;

    match game.get(&(x, y)) {
        Some(Floor) => get_seat_by_direction(game, x, y, x_diff, y_diff),
        Some(tile) => Some(tile),
        None => None
    }
}

fn get_next(game: &Game, x: isize, y: isize) -> Tile {
    let neighbors: isize = [
        get_seat_by_direction(game, x, y, -1,  0),
        get_seat_by_direction(game, x, y,  1,  0),
        get_seat_by_direction(game, x, y,  0, -1),
        get_seat_by_direction(game, x, y,  0,  1),
        get_seat_by_direction(game, x, y, -1, -1),
        get_seat_by_direction(game, x, y, -1,  1),
        get_seat_by_direction(game, x, y,  1, -1),
        get_seat_by_direction(game, x, y,  1,  1),
    ]
        .iter()
        .map(|t| match t {
            Some(Occupied) => 1,
            _ => 0,
        })
        .sum();

    match (game.get(&(x, y)), neighbors) {
        (Some(Empty), 0) => Occupied,
        (Some(Occupied), n) if n >= 5 => Empty,
        (Some(tile), _) => tile.to_owned(),
        _ => panic!("Seat does not exist: {:?}", (x, y))
    }
}

#[allow(dead_code)]
fn print_game(game: &Game, game_size: &(isize, isize)) {
    for y in 0..game_size.1 {
        let line: String = (0..game_size.0)
            .map(|x| game.get(&(x, y)).unwrap().to_string())
            .collect();
        eprintln!("{}", line);
    }

    eprintln!();
}

fn main() {
    let input: Vec<Vec<Tile>> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    let mut game: Game = input.iter().enumerate()
        .flat_map(|(y, seats)| {
            seats.iter().enumerate().map(move |(x, seat)| {
                ((x as isize, y as isize), seat.clone())
            })
        })
        .collect();

    let game_size = game.keys()
        .fold((0, 0), |acc, (x, y)| (max(acc.0, x + 1), max(acc.1, y + 1)));

    loop {
        // print_game(&game, &game_size);

        let mut next_game: Game = Game::new();

        for y in 0..game_size.1 {
            for x in 0..game_size.0 {
                next_game.insert((x, y), get_next(&game, x, y));
            }
        }

        if next_game == game {
            break;
        }

        game = next_game;
    }

    let num_occupied = game.values()
        .filter(|tile| tile == &&Occupied)
        .count();

    println!("{}", num_occupied);
}