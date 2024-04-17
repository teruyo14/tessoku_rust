use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let rc: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let g: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![];
    for _ in 0..rc[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<char> = input.trim().chars().collect();
        map.push(tmp);
    }

    let mut dist = vec![vec![-1; rc[1]]; rc[0]];
    dist[s[0] - 1][s[1] - 1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((s[0] - 1, s[1] - 1));

    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if 0 <= ny && ny < rc[0] as isize && 0 <= nx && nx < rc[1] as isize {
                if map[ny as usize][nx as usize] == '.'
                    && (dist[ny as usize][nx as usize] == -1
                        || dist[ny as usize][nx as usize] > dist[y][x] + 1)
                {
                    dist[ny as usize][nx as usize] = dist[y][x] + 1;
                    queue.push_back((ny as usize, nx as usize));
                }
            }
        }
    }

    println!("{}", dist[g[0] - 1][g[1] - 1]);
}
