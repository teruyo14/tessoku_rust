use std::cmp::min;
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

    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;
    for i in 0..n - 2 {
        dp[i + 1] = min(dp[i + 1], dp[i] + a[i]);
        dp[i + 2] = min(dp[i + 2], dp[i] + b[i]);
    }
    dp[n-1] = min(dp[n-1], dp[n-2] + a[n-2]);

    println!("{}", dp[n - 1]);
}
