use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pi = vec![0; n];
    pi[0] = a[0];
    for i in 1..n {
        pi[i] = pi[i - 1].max(a[i]);
    }

    let mut qi = vec![0; n];
    qi[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        qi[i] = qi[i + 1].max(a[i]);
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let d: usize = input.trim().parse().unwrap();

    for _ in 0..d {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lr: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let l = lr[0];
        let r = lr[1];

        let max_size = if l > 1 { pi[l - 2] } else { 0 }.max(qi[r]);

        println!("{}", max_size);
    }
}
