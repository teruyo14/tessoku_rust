use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    let s_bytes = s.as_bytes();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in (i + 1)..n {
            if s_bytes[i] == s_bytes[j] {
                dp[i][j] = if i + 1 <= j - 1 {
                    dp[i + 1][j - 1] + 2
                } else {
                    2
                };
            } else {
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}
