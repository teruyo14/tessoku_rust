use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nw: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![1_000_000_100; 100000 + 1]; nw[0] + 1];

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
        for j in 0..=100000 {
            if dp[i][j] == 1_000_000_100 {
                continue;
            }

            dp[i + 1][j] = min(dp[i][j], dp[i + 1][j]);
            if j + wv[i][1] <= 100000 {
                dp[i + 1][j + wv[i][1]] =
                    min(dp[i + 1][j + wv[i][1]], dp[i][j] + wv[i][0] as isize);
            }
        }
    }

    let mut ans = 0;
    for j in 0..=100000 {
        if dp[nw[0]][j] <= nw[1] as isize {
            ans = j;
        }
    }

    println!("{}", ans);
}
