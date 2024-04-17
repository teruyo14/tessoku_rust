use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nt: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![vec![]; nt[0]];
    for _ in 0..nt[0] - 1 {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let ab: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map[ab[0] - 1].push(ab[1] - 1);
        map[ab[1] - 1].push(ab[0] - 1);
    }

    let mut dp = vec![0; nt[0]];

    dfs(&map, &mut dp, usize::MAX, nt[1] - 1);

    let ans: Vec<String> = dp.iter().map(|r| r.to_string()).collect();
    println!("{}", ans.join(" "));
}

fn dfs(map: &Vec<Vec<usize>>, dp: &mut Vec<usize>, parent: usize, current: usize) -> usize {
    let mut max_depth = 0;
    for &adj in &map[current] {
        if adj != parent {
            let depth = dfs(map, dp, current, adj) + 1;
            if depth > max_depth {
                max_depth = depth;
            }
        }
    }
    dp[current] = max_depth;
    max_depth
}
