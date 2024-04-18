use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nl: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let k: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    a.push(0);
    a.push(nl[1]);
    a.sort();
    let mut ok = 1_000_000_100;
    let mut ng = 0;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let mut cnt = 0;
        let mut cur = 0;
        for i in 1..nl[0] + 2 {
            cur += a[i] - a[i - 1];
            if cur > mi {
                cnt += 1;
                cur = 0;
            }
        }
        if cnt > k {
            ng = mi;
        } else {
            ok = mi;
        }
    }

    println!("{}", ok);
}
