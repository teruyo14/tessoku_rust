use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ans = 0;
    for i in 1..=nk[0] {
        for j in 1..=nk[0] {
            if 1 <= nk[1] - i - j && nk[1] - i - j <= nk[0] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
