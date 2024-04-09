use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let t = input.trim().to_string();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    dp[0][0] = 0;
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s.chars().nth(i) == t.chars().nth(j) {
                dp[i + 1][j + 1] = max(dp[i][j] + 1, dp[i + 1][j + 1]);
            } else {
                dp[i + 1][j + 1] = max(max(dp[i + 1][j], dp[i][j + 1]), dp[i + 1][j + 1])
            }
        }
    }

    println!("{}", dp[s.len()][t.len()])
}
