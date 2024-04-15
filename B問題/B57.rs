use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![0; nk[0] + 1]; 30];

    for i in 0..=nk[0] {
        dp[0][i] = i - digit_sum(i);
    }

    for d in 0..29 {
        for i in 0..=nk[0] {
            dp[d + 1][i] = dp[d][dp[d][i]];
        }
    }

    for num in 1..=nk[0] {
        let mut tmp = num;
        for d in 0..30 {
            if (nk[1] & (1 << d)) != 0 {
                tmp = dp[d][tmp];
            }
        }
        println!("{}", tmp);
    }
}

fn digit_sum(mut num: usize) -> usize {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
