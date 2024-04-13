use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let abc: Vec<i128> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if abc[0] + abc[1] + abc[2] == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
