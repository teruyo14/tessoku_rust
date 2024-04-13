use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cnt = HashMap::new();
    for i in 0..n {
        *cnt.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    for &i in cnt.values() {
        ans += combination(i);
    }

    println!("{}", ans);
}

fn combination(n: u64) -> u64 {
    if n < 3 {
        return 0;
    }
    n * (n - 1) * (n - 2) / 6
}
