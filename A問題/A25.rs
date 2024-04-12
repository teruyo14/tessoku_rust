use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let hw: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut grid = vec![];
    for _ in 0..hw[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        grid.push(input.trim().to_string());
    }

    let mut dp: Vec<Vec<i128>> = vec![vec![0; hw[1]]; hw[0]];

    dp[0][0] = if grid[0].chars().nth(0).unwrap() == '.' {
        1
    } else {
        0
    };

    for i in 0..hw[0] {
        for j in 0..hw[1] {
            if grid[i].chars().nth(j).unwrap() != '#' {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
    }

    println!("{}", dp[hw[0] - 1][hw[1] - 1]);
}
