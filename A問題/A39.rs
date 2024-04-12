use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut lr: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let times: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let l = times[0];
        let r = times[1];
        lr.push((l, r));
    }

    lr.sort_by(|a, b| a.1.cmp(&(b.1)));

    let mut ans = 1;
    let mut t = lr[0].1;
    for i in 1..n {
        if lr[i].0 >= t {
            ans += 1;
            t = lr[i].1;
        }
    }

    println!("{}", ans);
}
