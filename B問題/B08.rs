use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut grid = vec![vec![0; 1501]; 1501];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let xy: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid[xy[1]][xy[0]] += 1;
    }

    let mut psum = vec![vec![0; 1501]; 1501];
    for i in 1..=1500 {
        for j in 1..=1500 {
            psum[i][j] = psum[i][j - 1] + psum[i - 1][j] - psum[i - 1][j - 1] + grid[i][j];
        }
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let abcd: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!(
            "{}",
            psum[abcd[3]][abcd[2]] - psum[abcd[1] - 1][abcd[2]] - psum[abcd[3]][abcd[0] - 1]
                + psum[abcd[1] - 1][abcd[0] - 1]
        );
    }
}
