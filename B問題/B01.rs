use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<u16> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", vec[0] + vec[1]);
}
