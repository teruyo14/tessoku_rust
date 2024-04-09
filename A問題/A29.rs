use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let ab: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a = ab[0];
    let b = ab[1];

    println!("{}", mod_pow(a, b, 1_000_000_007));
}

fn mod_pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % m;
        }
        b >>= 1;
        a = a * a % m;
    }
    result
}
