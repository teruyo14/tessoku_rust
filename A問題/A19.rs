use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nw: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![-1; nw[1] + 1]; nw[0] + 1];

    let mut wv = vec![];
    for _ in 0..nw[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        wv.push(tmp);
    }

    dp[0][0] = 0;
    for i in 0..nw[0] {
        for j in 0..=nw[1] {
            if dp[i][j] < 0 {
                continue;
            }

            dp[i + 1][j] = max(dp[i][j], dp[i + 1][j]);
            if j + wv[i][0] <= nw[1] {
                dp[i + 1][j + wv[i][0]] = max(dp[i + 1][j + wv[i][0]], dp[i][j] + wv[i][1] as isize);
            }
        }
    }

    let mut ans = -1;
    for j in 0..=nw[1] {
        ans = max(ans, dp[nw[0]][j])
    }

    println!("{}", ans);
}
