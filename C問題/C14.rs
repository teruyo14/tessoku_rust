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

    let mut abc = vec![];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        abc.push((tmp[0], tmp[1], tmp[2]));
    }

    let mut map = vec![vec![]; nm[0]];
    for (a, b, c) in abc {
        map[a - 1].push((b - 1, c));
        map[b - 1].push((a - 1, c));
    }

    let mut dist1 = vec![1_000_000_000; nm[0]];
    let mut dist2 = vec![1_000_000_000; nm[0]];
    let mut q = BinaryHeap::new();
    dist1[0] = 0;
    q.push((Reverse(dist1[0]), 0));
    while let Some((Reverse(d), v)) = q.pop() {
        if dist1[v] < d {
            continue;
        }
        for &(i, c) in &map[v] {
            if dist1[i] > dist1[v] + c {
                dist1[i] = dist1[v] + c;
                q.push((Reverse(dist1[i]), i));
            }
        }
    }

    dist2[nm[0] - 1] = 0;
    q.push((Reverse(dist2[nm[0] - 1]), nm[0] - 1));
    while let Some((Reverse(d), v)) = q.pop() {
        if dist2[v] < d {
            continue;
        }
        for &(i, c) in &map[v] {
            if dist2[i] > dist2[v] + c {
                dist2[i] = dist2[v] + c;
                q.push((Reverse(dist2[i]), i));
            }
        }
    }

    let mut ans = 0;
    for i in 0..nm[0] {
        if dist1[nm[0] - 1] == dist1[i] + dist2[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
