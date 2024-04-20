use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nmk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ab = vec![];
    for _ in 0..nmk[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ab.push((tmp[0], tmp[1]));
    }

    let mut score = vec![vec![0; nmk[0]]; nmk[0]];
    for (a, b) in ab {
        for i in 0..nmk[0] {
            for j in 0..=i {
                if j <= a - 1 && b - 1 <= i {
                    score[j][i] += 1;
                }
            }
        }
    }

    let mut dp = vec![vec![i32::MIN; nmk[0] + 1]; nmk[2] + 1];
    dp[0][0] = 0;
    for k in 0..nmk[2] {
        for i in 0..nmk[0] {
            for j in 0..=i {
                dp[k + 1][i + 1] = dp[k + 1][i + 1].max(dp[k][j] + score[j][i]);
            }
        }
    }

    println!("{}", dp[nmk[2]][nmk[0]]);
}
