use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut s = 1;
    let mut e = vec![0; nq[0] + 2];
    for i in 1..=nq[0] {
        e[i] = i;
    }

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let quary: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match quary[0] {
            1 => {
                if s == 1 {
                    e[quary[1]] = quary[2];
                }
                if s == 2 {
                    e[nq[0] + 1 - quary[1]] = quary[2];
                }
            }
            2 => {
                if s == 1 {
                    s = 2;
                } else {
                    s = 1;
                }
            }
            3 => {
                if s == 1 {
                    println!("{}", e[quary[1]]);
                }
                if s == 2 {
                    println!("{}", e[nq[0] + 1 - quary[1]])
                }
            }
            _ => (),
        }
    }
}
