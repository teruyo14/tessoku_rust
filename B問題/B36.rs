use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let mut on = 0;
    for ch in s.chars() {
        if ch == '1' {
            on += 1;
        }
    }

    if on % 2 == nk[1] % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
