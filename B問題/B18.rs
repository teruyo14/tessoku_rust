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
        let mut subset = vec![];
        let mut sum = ns[1];
        for i in (1..=ns[0]).rev() {
            if sum >= a[i - 1] && dp[i - 1][sum - a[i - 1]] {
                subset.push(i);
                sum -= a[i - 1];
            }
        }
        println!("{}", subset.len());
        for i in subset.iter().rev() {
            print!("{} ", i);
        }
        println!()
    } else {
        println!("-1");
    }
}
