use std::collections::BinaryHeap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nd: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut jobs = vec![vec![]; nd[1] + 1];
    for _ in 0..nd[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let xy: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if xy[0] <= nd[1] {
            jobs[xy[0]].push(xy[1])
        }
    }

    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for i in 1..=nd[1] {
        for j in &jobs[i] {
            heap.push(j);
        }

        if let Some(v) = heap.pop() {
            ans += v;
        }
    }

    println!("{}", ans);
}
