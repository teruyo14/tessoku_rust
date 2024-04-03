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

    let mut psum = vec![0];
    for i in 0..nq[0] {
        psum.push(psum.last().unwrap() + a[i]);
    }

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lr: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{}", psum[lr[1]] - psum[lr[0] - 1]);
    }
}
