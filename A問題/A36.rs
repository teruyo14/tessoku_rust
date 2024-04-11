use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if 2 * (nk[0] - 1) <= nk[1] && (nk[1] - 2 * (nk[0] - 1)) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
