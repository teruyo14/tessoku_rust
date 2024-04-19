use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut c: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    c.sort();

    let mut prefix: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + c[i - 1];
    }

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        let idx: usize = prefix.partition_point(|&xc| xc <= x);
        println!("{}", idx - 1);
    }
}
