use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
        let abc: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map[abc[0]].push((abc[1], abc[2]));
        map[abc[1]].push((abc[0], abc[2]));
    }

    let mut confirm = vec![false; nm[0] + 1];

    let mut cur = vec![usize::MAX; nm[0] + 1];
    cur[1] = 0;

    let mut hqueue = BinaryHeap::new();
    hqueue.push(Reverse((cur[1], 1)));
    while let Some(Reverse((_, pos))) = hqueue.pop() {
        if confirm[pos] {
            continue;
        }

        confirm[pos] = true;
        for (p, cost) in &map[pos] {
            if cur[*p] > cur[pos] + cost {
                cur[*p] = cur[pos] + cost;
                hqueue.push(Reverse((cur[*p], *p)));
            }
        }
    }

    for i in 1..=nm[0] {
        if cur[i] != usize::MAX {
            println!("{}", cur[i]);
        } else {
            println!("-1");
        }
    }
}
