use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![false; nk[0] + 1];
    for i in 0..=nk[0] {
        for j in 0..nk[1] {
            if i >= a[j] && dp[i - a[j]] == false {
                dp[i] = true;
            }
        }
    }

    if dp[nk[0]] {
        println!("First");
    } else {
        println!("Second");
    }
}
