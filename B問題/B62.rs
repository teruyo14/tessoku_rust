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
    let mut ans = vec![];
    if dfs(1, nm[0], &map, &mut visited, &mut ans) {
        println!(
            "{}",
            ans.iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn dfs(
    pos: usize,
    target: usize,
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut Vec<usize>,
) -> bool {
    ans.push(pos);
    visited[pos] = true;

    if pos == target {
        return true;
    }

    for &next in &map[pos] {
        if !visited[next] {
            if dfs(next, target, &map, visited, ans) {
                return true;
            }
        }
    }

    ans.pop();
    visited[pos] = false;
    false
}
