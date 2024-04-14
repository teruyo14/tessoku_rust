use std::collections::BTreeSet;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match query[0] {
            1 => {
                set.insert(query[1]);
            }
            2 => {
                set.remove(&query[1]);
            }
            3 => {
                if let Some(&v) = set.range(query[1]..).next() {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }
    }
}
