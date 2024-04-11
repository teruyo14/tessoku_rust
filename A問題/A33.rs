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

    let mut cnt = a[0];
    for i in 1..n {
        cnt ^= a[i];
    }

    if cnt != 0 {
        println!("First");
    } else {
        println!("Second")
    }
}
