use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nab: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![false; nab[0] + 1];
    for i in 0..=nab[0] {
        if i >= nab[1] && dp[i - nab[1]] == false {
            dp[i] = true;
        }
        if i >= nab[2] && dp[i - nab[2]] == false {
            dp[i] = true;
        }
    }

    if dp[nab[0]] {
        println!("First");
    } else {
        println!("Second");
    }
}
