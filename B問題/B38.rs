use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let mut h = vec![1; n];
    let mut cnt = 1;
    for i in 0..n - 1 {
        if s.chars().nth(i) == Some('A') {
            cnt += 1;
        } else {
            cnt = 1;
        }

        h[i + 1] = max(h[i + 1], cnt);
    }

    let mut cnt = 1;
    for i in (0..n-1).rev() {
        if s.chars().nth(i) == Some('A') {
            cnt = 1;
        } else {
            cnt += 1;
        }
        h[i] = max(h[i], cnt);
    }

    let mut ans = 0;
    for i in h {
        ans += i;
    }
    println!("{}",ans);
}
