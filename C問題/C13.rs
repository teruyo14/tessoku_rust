use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let np: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cnt = HashMap::new();
    let m = 1_000_000_007;
    for &i in &a {
        cnt.entry(i % m).and_modify(|x| *x += 1).or_insert(1);
    }

    let ans = cnt.iter().fold(0, |tmp, (&k, &v)| {
        let x = np[1] * pow(k, m - 2, m) % m;
        if k == 0 {
            tmp + v * (v - 1) / 2 + v * (np[0] - v)
        } else if k == x {
            tmp + v * (v - 1) / 2
        } else if k < x {
            tmp + v * cnt.get(&x).unwrap_or(&0)
        } else {
            tmp
        }
    });

    println!("{}", ans);
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
