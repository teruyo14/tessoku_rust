use std::cmp::min;
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
        let query: Vec<i128> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match query[0] {
            1 => {
                set.insert(query[1]);
            }

            2 => {
                let mut cnt = i128::MAX;
                if let Some(&v) = set.range(query[1]..).next() {
                    cnt = min(cnt, (v - query[1]).abs());
                }

                if let Some(&v) = set.range(..query[1]).next_back() {
                    cnt = min(cnt, (query[1] - v).abs());
                }

                if cnt == i128::MAX {
                    println!("-1");
                } else {
                    println!("{}", cnt);
                }
            }
            _ => (),
        }
    }
}
