use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut incorrect = vec![0; nm[0] + 1];
    for i in 0..nm[1] {
        incorrect[a[i]] += 1;
    }

    for i in 1..=nm[0] {
        println!("{}", nm[1] - incorrect[i]);
    }
}
