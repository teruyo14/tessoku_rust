use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x.push(tmp[0]);
        y.push(tmp[1]);
    }

    let mut dp = vec![vec![f64::INFINITY; n]; 2usize.pow(n as u32)];
    dp[1][0] = 0.0;

    for i in 1..2usize.pow(n as u32) {
        for j in 0..n {
            if dp[i][j] < f64::INFINITY {
                for k in 0..n {
                    if i & (1 << k) == 0 {
                        let d = (((x[j] as i32 - x[k] as i32).pow(2)
                            + (y[j] as i32 - y[k] as i32).pow(2))
                            as f64)
                            .sqrt();
                        let next_state = i | (1 << k);
                        dp[next_state][k] = f64::min(dp[next_state][k], dp[i][j] + d);
                    }
                }
            }
        }
    }

    let mut result = f64::INFINITY;
    for j in 0..n {
        if n > 1 {
            let d = (((x[j] as i32 - x[0] as i32).pow(2) + (y[j] as i32 - y[0] as i32).pow(2))
                as f64)
                .sqrt();
            result = f64::min(result, dp[(1 << n) - 1][j] + d);
        }
    }

    println!("{}", result);
}
