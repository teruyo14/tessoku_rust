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
    let a: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ans = 0;
    let mut left = 0;
    for right in 1..nk[0] {
        while a[right] - a[left] > nk[1] as isize {
            left += 1;
        }
        ans += right - left;
    }
    println!("{}", ans);
}
