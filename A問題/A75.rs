use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut td = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        td.push((tmp[0], tmp[1]));
    }

    td.sort_by(|x, y| x.1.cmp(&y.1));

    let mut dp = vec![0; 1441];
    dp[0] = 1;
    for i in 0..n {
        let (t, d) = td[i];
        let mut dp_next = vec![0; 1441];
        for j in 0..1441 {
            if dp[j] != 0 {
                dp_next[j] = dp_next[j].max(dp[j]);
                if j + t <= d {
                    dp_next[j + t] = dp[j] + 1;
                }
            }
        }
        dp = dp_next;
    }

    println!("{}", dp.into_iter().max().unwrap() - 1);
}
