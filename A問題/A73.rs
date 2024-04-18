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
        let abcd: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map[abcd[0]].push((abcd[1], 10_000 * abcd[2] - abcd[3]));
        map[abcd[1]].push((abcd[0], 10_000 * abcd[2] - abcd[3]));
    }

    let mut confirm = vec![false; nm[0] + 1];
    let mut cur = vec![usize::MAX; nm[0] + 1];
    cur[1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((cur[1], 1)));

    while let Some(Reverse((_, pos))) = heap.pop() {
        if confirm[pos] {
            continue;
        }

        confirm[pos] = true;
        for (next, cost) in &map[pos] {
            if cur[*next] > cur[pos] + cost {
                cur[*next] = cur[pos] + cost;
                heap.push(Reverse((cur[*next], *next)))
            }
        }
    }

    let dist = (cur[nm[0]] + 9999) / 10_000;
    let num = dist * 10_000 - cur[nm[0]];
    println!("{} {}", dist, num);
}
