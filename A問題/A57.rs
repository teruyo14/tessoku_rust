use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![vec![0; nq[0]]; 30];

    for i in 0..nq[0] {
        dp[0][i] = a[i] - 1;
    }

    for d in 0..29 {
        for i in 0..nq[0] {
            dp[d + 1][i] = dp[d][dp[d][i]];
        }
    }

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let xy: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut tmp = xy[0] - 1;
        for d in (0..30).rev() {
            if ((xy[1] >> d) & 1) == 1 {
                tmp = dp[d][tmp];
            }
        }
        println!("{}", tmp + 1);
    }
}
