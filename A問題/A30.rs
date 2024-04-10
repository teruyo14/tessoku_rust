use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nr: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let m = 1_000_000_007;
    let a = fact(nr[0]) % m;
    let b = (fact(nr[1]) * fact(nr[0] - nr[1])) % m;

    println!("{}", a * mod_pow(b, m - 2, m) % m);
}

fn fact(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n % 1_000_000_007
    }
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
