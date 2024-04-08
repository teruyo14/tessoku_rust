use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pairs: Vec<(usize, usize)> =
        a.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
    pairs.sort_unstable();

    let mut rank = 0;
    let mut prev = None;
    let mut mapping = HashMap::new();
    for (value, index) in pairs {
        if prev != Some(value) {
            rank += 1;
            prev = Some(value);
        }
        mapping.insert(index, rank);
    }

    let mut b: Vec<usize> = vec![0; n];
    for (index, &rank) in mapping.iter() {
        b[*index] = rank;
    }

    for value in b {
        print!("{} ", value);
    }
    println!();
}
