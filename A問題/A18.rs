use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let ns: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![false; ns[1] + 1]; ns[0] + 1];

    dp[0][0] = true;
    for i in 0..ns[0] {
        for j in 0..=ns[1] {
            if !dp[i][j] {
                continue;
            }

            dp[i + 1][j] = true;
            if j + a[i] <= ns[1] {
                dp[i + 1][j + a[i]] = true;
            }
        }
    }

    if dp[ns[0]][ns[1]] {
        println!("Yes");
    } else {
        println!("No");
    }
}
