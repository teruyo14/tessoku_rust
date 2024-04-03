use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let d: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut imos = vec![0; d + 2];

    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lr: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        imos[lr[0]] += 1;
        imos[lr[1] + 1] -= 1;
    }

    let mut psum = vec![0];
    for i in 1..d + 1 {
        psum.push(psum.last().unwrap() + imos[i]);
    }

    for i in 1..=d {
        println!("{}", psum[i]);
    }
}
