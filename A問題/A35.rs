use std::cmp::{max, min};
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for j in 1..n + 1 {
        dp[n][j] = a[j - 1];
    }

    for i in (1..n).rev() {
        for j in 1..=i {
            if i % 2 == 1 {
                dp[i][j] = max(dp[i + 1][j], dp[i + 1][j + 1]);
            } else {
                dp[i][j] = min(dp[i + 1][j], dp[i + 1][j + 1]);
            }
        }
    }

    println!("{}", dp[1][1]);
}
