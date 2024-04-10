use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let d3 = n / 3;
    let d5 = n / 5;
    let d7 = n / 7;
    let d15 = n / 15;
    let d21 = n / 21;
    let d35 = n / 35;
    let d105 = n / 105;

    println!("{}", d3 + d5 + d7 - d15 - d21 - d35 + d105);
}
