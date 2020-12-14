use std::io::{self, BufRead};
use crate::Bit::{Zero, One, Any};
use std::collections::HashSet;

use derive_more::{IntoIterator, Index, IndexMut, Deref};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum Bit {
    Zero,
    One,
    Any,
}

impl From<char> for Bit {
    fn from(c: char) -> Self {
        match c {
            '0' => Zero,
            '1' => One,
            'X' => Any,
            _ => panic!("Unrecognized Bit: {}", c)
        }
    }
}

impl From<Bit> for char {
    fn from(b: Bit) -> Self {
        match b {
            Zero => '0',
            One => '1',
            Any => 'X',
        }
    }
}

#[derive(Debug, IntoIterator, Clone, Eq, PartialEq, Hash, Index, IndexMut, Deref)]
struct Address(Vec<Bit>);

impl Address {
    fn new() -> Self {
        Address(vec![Zero; 36])
    }
}

impl FromIterator<Bit> for Address {
    fn from_iter<T: IntoIterator<Item=Bit>>(iter: T) -> Self {
        let mut bits: Vec<Bit> = vec![];

        for bit in iter {
            bits.push(bit);
        }

        Address(bits)
    }
}

#[derive(Debug)]
enum AddressParseError {
    InvalidFormat
}

impl FromStr for Address {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            if let Ok(address) = s.parse::<u32>() {
                return format!("{:036b}", address).parse();
            }

            return Err(AddressParseError::InvalidFormat);
        }

        let address = s.chars()
            .map(|c| Bit::from(c))
            .collect();
        return Ok(address)
    }
}

impl Address {
    fn mask(&self, mask: &Address) -> Address {
        self.iter().zip(mask.iter())
            .map(|(a, m)| match (a, m) {
                (_, Any) => Any,
                (_, One) => One,
                (a, Zero) => *a
            })
            .collect()
    }

    fn concretize(&self) -> HashSet<Address> {
        let anys: Vec<usize> = self.iter()
            .enumerate()
            .filter(|(_, b)| b == &&Any)
            .map(|(i, _)| i)
            .collect();
        let mut addresses = HashSet::new();

        for i in 0..(2usize.pow(anys.len() as u32)) {
            let mut address = self.clone();

            for j in 0..anys.len() {
                address[anys[anys.len() - (j + 1)]] = match (i >> j) & 1 {
                    0 => Zero,
                    1 => One,
                    _ => unreachable!()
                }
            }

            addresses.insert(address);
        }

        addresses
    }
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut mask: Address = Address::new();
    let mut writes: Vec<(Address, u64)> = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split(" = ").collect();
        let command = parts[0];
        let value = parts[1];

        if command == "mask" {
            mask = value.parse().unwrap();
        } else {
            let address  = command.split(|c| c == '[' || c == ']').collect::<Vec<&str>>()[1];
            let address: Address = address.parse().unwrap();
            let address = address.mask(&mask);
            writes.push((address, value.parse::<u64>().unwrap()));
        }
    }

    let mut sum = 0;
    let mut visited: HashSet<Address> = HashSet::new();

    for (address, value) in writes.iter().rev() {
        let concrete_addresses = address.concretize();

        for a in concrete_addresses {
            if !visited.contains(&a) {
                sum += value;
                visited.insert(a);
            }
        }
    }

    println!("{}", sum);
}