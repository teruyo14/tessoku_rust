use std::cmp::max;
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

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let b: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![-1_000_000_000; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[a[i] - 1] = max(dp[i] + 100, dp[a[i] - 1]);
        dp[b[i] - 1] = max(dp[i] + 150, dp[b[i] - 1]);
    }

    println!("{}", dp[n - 1]);
}
