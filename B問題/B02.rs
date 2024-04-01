use std::io::stdin;
use std::process::exit;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let ab: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in ab[0]..=ab[1] {
        if 100 % i == 0 {
            println!("Yes");
            exit(0);
        }
    }

    println!("No");
}
