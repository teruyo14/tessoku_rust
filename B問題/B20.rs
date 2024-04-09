use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let t = input.trim().to_string();

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        dp[i][0] = i;
    }
    for j in 0..=t.len() {
        dp[0][j] = j;
    }

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            let cost = if s_chars[i - 1] == t_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = min(
                min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
