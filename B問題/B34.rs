use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nxy: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cnt = 0;
    let grundy = [0, 0, 1, 1, 2];
    for i in 0..nxy[0] {
        cnt ^= grundy[a[i] % 5];
    }

    if cnt != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
