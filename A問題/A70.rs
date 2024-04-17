use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut actions = vec![];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let xyz: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        actions.push((xyz[0], xyz[1], xyz[2]));
    }

    let s: usize = a
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc + ((val as usize) << i));
    let g: usize = (1 << nm[0]) - 1;

    let mut dist = vec![-1; 1 << nm[0]];
    dist[s] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(s);

    while let Some(pos) = queue.pop_front() {
        for &(x, y, z) in &actions {
            let next = pos ^ (1 << x) ^ (1 << y) ^ (1 << z);
            if dist[next] == -1 {
                dist[next] = dist[pos] + 1;
                queue.push_back(next);
            }
        }
    }

    println!("{}", dist[g]);
}
