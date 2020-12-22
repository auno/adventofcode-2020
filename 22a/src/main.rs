use std::io::{self, BufRead};
use std::collections::VecDeque;

fn read_deck(input: &mut std::slice::Iter<String>) -> VecDeque<u32> {
    let mut deck: VecDeque<u32> = VecDeque::new();
    input.next();

    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        deck.push_back(line.parse().unwrap())
    }

    deck
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut input = input.iter();

    let mut decks = vec![
        read_deck(&mut input),
        read_deck(&mut input)
    ];

    while decks.iter().all(|deck| !deck.is_empty()) {
        let mut cards: Vec<(usize, u32)> = decks.iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .enumerate()
            .collect();

        cards.sort_by_key(|card| card.1);
        cards.reverse();

        let winner = cards[0].0;

        cards.iter()
            .for_each(|(_, card)| decks[winner].push_back(*card));
    }

    let winner = decks.iter()
        .fold(VecDeque::new(), |acc, deck| {
            if acc.len() > deck.len() {
                acc
            } else {
                deck.clone()
            }
        });

    let score = winner.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, value)| acc + (value * (i as u32 + 1)));

    println!("{}", score);
}
