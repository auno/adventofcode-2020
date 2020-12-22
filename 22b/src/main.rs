use std::io::{self, BufRead};
use std::collections::{VecDeque, HashSet};

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

fn play(decks: &mut Vec<VecDeque<u32>>) -> usize {
    let mut seen: Vec<HashSet<VecDeque<u32>>> = vec![
        HashSet::new(),
        HashSet::new()
    ];

    while decks.iter().all(|deck| !deck.is_empty()) {
        if decks.iter()
            .zip(seen.iter())
            .all(|(deck, seen)| seen.contains(deck)) {
            return 0;
        }

        decks.iter()
            .zip(seen.iter_mut())
            .for_each(|(deck, seen)| {
                seen.insert(deck.clone());
            });

        let mut cards: Vec<(usize, u32)> = decks.iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .enumerate()
            .collect();

        let winner = if cards.iter()
            .zip(decks.iter())
            .all(|((_i, card), deck)| deck.len() as u32 >= *card) {

            let mut new_decks = cards.iter()
                .zip(decks.iter())
                .map(|((_i, card), deck)| deck.iter().take(*card as usize).cloned().collect())
                .collect();

            let winner = play(&mut new_decks);

            if winner == 1 {
                // This "shortcut" is probably the only part that assumes only two players
                cards.reverse();
            }

            winner
        } else {
            cards.sort_by_key(|card| card.1);
            cards.reverse();

            cards[0].0
        };

        cards.iter().for_each(|(_, card)| decks[winner].push_back(*card));
    }

    let winner = decks.iter()
        .map(|deck| deck.len())
        .enumerate()
        .max_by_key(|(_i, len)| *len)
        .map(|(i, _)| i)
        .unwrap();

    winner
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

    let winner = play(&mut decks);

    let score = decks[winner].iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, value)| acc + (value * (i as u32 + 1)));

    println!("{}", score);
}
