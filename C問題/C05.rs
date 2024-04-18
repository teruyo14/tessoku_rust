use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut n: usize = input.trim().parse().unwrap();

    n -= 1;

    for i in (0..10).rev() {
        print!("{}", if n & (1 << i) > 0 { 7 } else { 4 });
    }

    println!();
}
