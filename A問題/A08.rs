use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let hw: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut x = vec![];
    for _ in 0..hw[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x.push(row);
    }

    let mut psum = vec![vec![0; hw[1] + 1]; hw[0] + 1];
    for i in 1..=hw[0] {
        for j in 1..=hw[1] {
            psum[i][j] = psum[i][j - 1] + x[i - 1][j - 1];
        }
    }

    for j in 1..=hw[1] {
        for i in 1..=hw[0] {
            psum[i][j] += psum[i - 1][j]
        }
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q = input.trim().parse().unwrap();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let abcd: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!(
            "{}",
            psum[abcd[2]][abcd[3]] - psum[abcd[2]][abcd[1] - 1] - psum[abcd[0] - 1][abcd[3]]
                + psum[abcd[0] - 1][abcd[1] - 1]
        );
    }
}
