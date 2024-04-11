use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut a = vec![];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<u8> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        a.push(tmp);
    }

    let mut dp = vec![vec![1_000_000_000; 2usize.pow(nm[0] as u32)]; nm[1] + 1];
    dp[0][0] = 0;
    for i in 1..=nm[1] {
        for j in 0..2usize.pow(nm[0] as u32) {
            let mut already = vec![-1; nm[0]];
            for k in 0..nm[0] {
                if (j / (2usize.pow(k as u32))) % 2 == 0 {
                    already[k] = 0;
                } else {
                    already[k] = 1;
                }
            }

            let mut v = 0;
            for k in 0..nm[0] {
                if already[k] == 1 || a[i - 1][k] == 1 {
                    v += 2usize.pow(k as u32);
                }
            }

            dp[i][j] = min(dp[i][j], dp[i - 1][j]);
            dp[i][v] = min(dp[i][v], dp[i - 1][j] + 1);
        }
    }

    if dp[nm[1]][2usize.pow((nm[0]) as u32) - 1] == 1_000_000_000 {
        println!("-1");
    } else {
        println!("{}", dp[nm[1]][2usize.pow((nm[0]) as u32) - 1]);
    }
}
