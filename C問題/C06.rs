use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    println!("{}", n);
    for i in 0..n {
        println!("{} {}", i + 1, (i + 1) % n + 1);
    }
}
