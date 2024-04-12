use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let dn: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut t = vec![24; dn[0]];
    for _ in 0..dn[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lrh: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for i in lrh[0]..=lrh[1] {
            t[i - 1] = min(t[i - 1], lrh[2]);
        }
    }

    let mut ans = 0;
    for i in t {
        ans += i;
    }
    println!("{}", ans);
}
