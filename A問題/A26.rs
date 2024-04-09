use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut prime_tf = vec![false; 300010];
    prime_tf[0] = true;
    prime_tf[1] = true;
    for i in 2..300010 {
        if prime_tf[i] {
            continue;
        }
        for j in (2 * i..300010).step_by(i) {
            prime_tf[j] = true;
        }
    }

    let mut prime = vec![];
    for i in 0..300010 {
        if !prime_tf[i] {
            prime.push(i);
        }
    }

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        if prime.binary_search(&x).is_ok() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
