use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nx: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut a: Vec<char> = input.trim().chars().collect();

    let mut queue = VecDeque::new();
    queue.push_back(nx[1] - 1);
    a[nx[1] - 1] = '@';

    while let Some(v) = queue.pop_front() {
        if v > 0 && a[v - 1] == '.' {
            a[v - 1] = '@';
            queue.push_back(v - 1);
        }
        if v + 1 < nx[0] && a[v + 1] == '.' {
            a[v + 1] = '@';
            queue.push_back(v + 1);
        }
    }

    for i in 0..nx[0] {
        print!("{}", a[i]);
    }
    println!();
}
