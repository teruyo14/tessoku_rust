use std::collections::VecDeque;
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

    let mut dist = vec![-1; nm[0] + 1];
    dist[1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(v) = queue.pop_front() {
        for next in &map[v] {
            if dist[*next] == -1 {
                dist[*next] = dist[v] + 1;
                queue.push_back(*next);
            }
        }
    }

    for i in 1..=nm[0] {
        println!("{}", dist[i]);
    }
}
