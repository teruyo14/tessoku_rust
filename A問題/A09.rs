use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let hwn: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut imos = vec![vec![0; hwn[1] + 2]; hwn[0] + 2];

    for _ in 0..hwn[2] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let abcd: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        imos[abcd[0] - 1][abcd[1] - 1] += 1;
        imos[abcd[0] - 1][abcd[3]] -= 1;
        imos[abcd[2]][abcd[1] - 1] -= 1;
        imos[abcd[2]][abcd[3]] += 1;
    }

    let mut psum = vec![vec![0; hwn[1] + 2]; hwn[0] + 2];
    for i in 1..=hwn[0] {
        for j in 1..=hwn[1] {
            psum[i][j] = psum[i - 1][j] + psum[i][j - 1] - psum[i - 1][j - 1] + imos[i - 1][j - 1];
        }
    }

    for i in 1..=hwn[0] {
        for j in 1..=hwn[1] {
            if j == hwn[1] {
                print!("{}", psum[i][j]);
            } else {
                print!("{} ", psum[i][j]);
            }
        }
        println!("");
    }
}
