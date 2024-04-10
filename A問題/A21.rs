use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut pa = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        pa.push(tmp);
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[1][n] = 0;
    for i in 1..=n {
        for j in (1..=n).rev() {
            let mut score1 = 0;
            let mut score2 = 0;
            if 2 <= i && i <= pa[i - 2][0] && pa[i - 2][0] <= j {
                score1 = pa[i - 2][1];
            }

            if j < n && i <= pa[j][0] && pa[j][0] <= j {
                score2 = pa[j][1];
            }

            if 2 <= i && j < n {
                dp[i][j] = max(dp[i - 1][j] + score1, dp[i][j + 1] + score2);
            } else if 2 <= i {
                dp[i][j] = dp[i - 1][j] + score1;
            } else if j < n {
                dp[i][j] = dp[i][j + 1] + score2;
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i == j {
                ans = max(ans, dp[i][j]);
            }
        }
    }

    println!("{}", ans);
}
