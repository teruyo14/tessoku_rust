use std::io::stdin;
use std::process::exit;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nx: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..nx[0] {
        if a[i] == nx[1] {
            println!("Yes");
            exit(0);
        }
    }

    println!("No");
}
