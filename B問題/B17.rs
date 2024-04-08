use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let h: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![0; n];
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = min(
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
        )
    }

    let mut ans = vec![n];
    let mut cnt = n - 1;
    while cnt > 1 {
        if dp[cnt - 1] + (h[cnt] - h[cnt - 1]).abs() < dp[cnt - 2] + (h[cnt] - h[cnt - 2]).abs() {
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
