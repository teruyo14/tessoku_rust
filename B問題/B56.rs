use std::io::stdin;

const BASE: u64 = 31;
const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let (forward_hashes, backward_hashes, powers) = precompute_hashes(s, nq[0]);

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lr: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if is_palindrome(
            &forward_hashes,
            &backward_hashes,
            &powers,
            lr[0] - 1,
            lr[1] - 1,
        ) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn precompute_hashes(s: &str, n: usize) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let mut forward_hashes = vec![0; n + 1];
    let mut backward_hashes = vec![0; n + 1];
    let mut powers = vec![1; n + 1];

    for i in 0..n {
        forward_hashes[i + 1] = (forward_hashes[i] * BASE + s.as_bytes()[i] as u64) % MOD;
        powers[i + 1] = (powers[i] * BASE) % MOD;
    }

    for i in (0..n).rev() {
        backward_hashes[i] = (backward_hashes[i + 1] * BASE + s.as_bytes()[i] as u64) % MOD;
    }

    (forward_hashes, backward_hashes, powers)
}

fn is_palindrome(
    forward_hashes: &Vec<u64>,
    backward_hashes: &Vec<u64>,
    powers: &Vec<u64>,
    l: usize,
    r: usize,
) -> bool {
    let forward_hash =
        (forward_hashes[r + 1] + MOD - (forward_hashes[l] * powers[r - l + 1]) % MOD) % MOD;
    let backward_hash =
        (backward_hashes[l] + MOD - (backward_hashes[r + 1] * powers[r - l + 1]) % MOD) % MOD;

    forward_hash == backward_hash
}
