use itertools::Itertools;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![vec![]; nm[0] + 1];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let ab: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map[ab[0]].push(ab[1]);
        map[ab[1]].push(ab[0]);
    }

    for i in 1..=nm[0] {
        println!("{}: {{{:?}}}", i, map[i].iter().join(", "));
    }
}
