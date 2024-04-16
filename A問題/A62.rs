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

    let mut visited = vec![false; nm[0] + 1];
    dfs(1, &map, &mut visited);

    if visited[1..=nm[0]].iter().all(|&x| x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(pos: usize, map: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    let mut stack = vec![];
    stack.push(pos);
    visited[pos] = true;

    while let Some(v) = stack.pop() {
        for &i in &map[v] {
            if !visited[i] {
                visited[i] = true;
                stack.push(i);
            }
        }
    }
}
