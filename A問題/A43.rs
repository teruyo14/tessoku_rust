use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nl: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ans = 0;
    for _ in 0..nl[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<&str> = input.split_whitespace().collect();
        let cnt: isize = tmp[0].parse().unwrap();
        if tmp[1] == "E" {
            ans = max(ans, nl[1] - cnt);
        } else {
            ans = max(ans, cnt);
        }
    }

    println!("{}", ans);
}
