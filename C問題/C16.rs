use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nmk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut asbt = vec![];
    for _ in 0..nmk[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        asbt.push((tmp[0] - 1, tmp[1], tmp[2] - 1, tmp[3] + nmk[2]));
    }

    asbt.sort_by(|a, b| a.3.cmp(&b.3));

    let mut dp = vec![vec![(0, 0)]; nmk[0]];
    for (a, s, b, t) in asbt {
        let p = dp[a].partition_point(|pred| pred.0 <= s);
        let count = (dp[a][p - 1].1 + 1).max(dp[b].last().unwrap().1);
        dp[b].push((t, count));
    }

    let ans = dp.iter().map(|s| s.last().unwrap().1).max().unwrap();
    println!("{}", ans);
}
