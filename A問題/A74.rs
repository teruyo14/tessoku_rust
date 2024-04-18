use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut p = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        p.push(tmp);
    }

    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if p[i][j] != 0 {
                x[j] = p[i][j];
                y[i] = p[i][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if x[i] > x[j] {
                ans += 1;
            }
            if y[i] > y[j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
