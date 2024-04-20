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

    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[i + 1] = max(dp[i + 1], dp[i]);
        if i + 2 <= n {
            dp[i + 2] = max(dp[i + 2], dp[i] + a[i]);
        }
    }

    println!("{}", dp[n]);
}
