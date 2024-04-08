use std::cmp::min;
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

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let b: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![0; n];
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    let mut ans = vec![n];
    let mut cnt = n - 1;
    while cnt > 1 {
        if dp[cnt - 1] + a[cnt - 1] < dp[cnt - 2] + b[cnt - 2] {
            ans.push(cnt);
            cnt -= 1;
        } else {
            ans.push(cnt - 1);
            cnt -= 2;
        }
    }

    if cnt == 1 {
        ans.push(1);
    }

    let t = ans.len();
    ans.sort();
    println!("{:}", t);
    for i in 0..t - 1 {
        print!("{} ", ans[i]);
    }
    println!("{}", ans[t - 1])
}
