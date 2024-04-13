use std::collections::HashMap;
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

    let mut cnt = vec![0; 100];
    for i in 0..n {
        cnt[a[i] % 100] += 1;
    }

    let mut ans = 0;
    for i in 1..50 {
        ans += cnt[i] * cnt[100 - i];
    }

    ans += cnt[0] * (cnt[0] - 1) / 2;
    ans += cnt[50] * (cnt[50] - 1) / 2;

    println!("{}", ans);
}
