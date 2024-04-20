use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let w: usize = input.trim().parse().unwrap();
    let m = 1_000_000_007;
    println!("{}", 12 * pow(7, w - 1, m) % m);
}

fn pow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res *= a;
            res %= m;
        }
        a *= a;
        a %= m;
        n /= 2;
    }
    res
}
