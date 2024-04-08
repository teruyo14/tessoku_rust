use std::io::stdin;
use std::process::exit;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let b: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let c: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let d: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut x = vec![];
    for i in 0..nk[0] {
        for j in 0..nk[0] {
            x.push(a[i] + b[j]);
        }
    }

    let mut y = vec![];
    for i in 0..nk[0] {
        for j in 0..nk[0] {
            y.push(c[i] + d[j]);
        }
    }

    y.sort();

    for i in 0..nk[0] * nk[0] {
        match y.binary_search(&(nk[1] - x[i])) {
            Ok(_) => {
                println!("Yes");
                exit(0);
            }
            Err(_) => continue,
        }
    }

    println!("No");
}
