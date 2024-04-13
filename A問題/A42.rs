use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ab = vec![];
    for _ in 0..nk[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ab.push(tmp);
    }

    let mut ans = 0;
    for a in 1..=100 {
        for b in 1..=100 {
            let mut cnt = 0;
            for i in 0..nk[0] {
                if a <= ab[i][0] && ab[i][0] <= a + nk[1] && b <= ab[i][1] && ab[i][1] <= b + nk[1]
                {
                    cnt += 1;
                }
            }
            ans = max(ans, cnt);
        }
    }

    println!("{}", ans);
}
