use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut prime_tf = vec![false; n + 1];
    prime_tf[0] = true;
    prime_tf[1] = true;
    for i in 2..n + 1 {
        if prime_tf[i] {
            continue;
        }
        for j in (2 * i..n + 1).step_by(i) {
            prime_tf[j] = true;
        }
    }

    let mut prime = vec![];
    for i in 0..n + 1 {
        if !prime_tf[i] {
            prime.push(i);
        }
    }

    for i in prime {
        println!("{}", i);
    }
}
