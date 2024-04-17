use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![vec![]; n + 1];
    for i in 2..=n {
        map[a[i - 2]].push(i);
    }

    let mut dp = vec![0; n + 1];
    for i in (1..=n).rev() {
        for j in &map[i] {
            dp[i] += dp[*j] + 1;
        }
    }

    for i in 1..=n {
        print!("{} ", dp[i]);
    }
    println!("");
}
