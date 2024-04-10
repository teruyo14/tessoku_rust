use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let d3 = n / 3;
    let d5 = n / 5;
    let d15 = n / 15;

    println!("{}", d3 + d5 - d15);
}
