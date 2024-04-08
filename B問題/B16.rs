use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let h: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![0; n];
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = min(
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
        )
    }

    println!("{}", dp.last().unwrap());
}
