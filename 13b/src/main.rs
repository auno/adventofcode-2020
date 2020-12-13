use std::io::{self, BufRead};

fn modinv(a: i128, n: i128) -> i128 {
    let (mut t, mut newt) = (0, 1);
    let (mut r, mut newr) = (n, a);

    while newr != 0 {
        let q = r / newr;

        let temp = t;
        t = newt;
        newt = temp - q * newt;

        let temp = r;
        r = newr;
        newr = temp - q * newr;
    }

    if r > 1 {
        panic!("{} is not invertible mod {}", a, n);
    }

    if t < 0 {
        t += n;
    }

    t
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let input: Vec<(i128, i128)> = input[1].split(',')
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| {
            (i, s.parse::<i128>().unwrap())
        })
        .map(|(i, x)| ((x - i as i128) % x, x))
        .collect();

    let mut acc: i128 = 0;
    let mut mul: i128 = 1;

    for (remainder, modulo) in input {
        let a = (remainder - acc).rem_euclid(modulo);
        let inverse = modinv(mul, modulo);
        let b = (a * inverse).rem_euclid(modulo);
        acc = acc + mul * b;
        mul = mul * modulo;
    }

    println!("{}", acc);
}