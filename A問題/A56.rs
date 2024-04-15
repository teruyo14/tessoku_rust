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

    let (hashes, powers) = precompute_hashes(&s, nq[0]);

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let idx: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if check_equal(
            &hashes,
            &powers,
            idx[0] - 1,
            idx[1] - 1,
            idx[2] - 1,
            idx[3] - 1,
        ) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn precompute_hashes(s: &str, n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut hashes = vec![0; n + 1];
    let mut powers = vec![1; n + 1];

    for i in 0..n {
        hashes[i + 1] = (hashes[i] * BASE + s.as_bytes()[i] as u64) % MOD;
        powers[i + 1] = (powers[i] * BASE) % MOD;
    }

    (hashes, powers)
}

fn check_equal(
    hashes: &Vec<u64>,
    powers: &Vec<u64>,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
) -> bool {
    let len1 = b - a + 1;
    let len2 = d - c + 1;

    if len1 != len2 {
        return false;
    }

    let hash1 = (hashes[b + 1] + MOD - (hashes[a] * powers[len1]) % MOD) % MOD;
    let hash2 = (hashes[d + 1] + MOD - (hashes[c] * powers[len1]) % MOD) % MOD;

    hash1 == hash2
}
