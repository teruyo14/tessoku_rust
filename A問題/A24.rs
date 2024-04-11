use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut l: Vec<usize> = vec![];

    for &value in &a {
        match l.binary_search(&value) {
            Ok(_) => (),
            Err(pos) => {
                if pos >= l.len() {
                    l.push(value);
                } else {
                    l[pos] = value;
                }
            }
        }
    }

    println!("{}", l.len());
}
