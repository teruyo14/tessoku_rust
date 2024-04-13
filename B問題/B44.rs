use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        a.push(tmp);
    }

    let mut t = vec![];
    for i in 0..n {
        t.push(i);
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();
    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let quary: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match quary[0] {
            1 => {
                let x = t[quary[1] - 1];
                t[quary[1] - 1] = t[quary[2] - 1];
                t[quary[2] - 1] = x;
            }
            2 => {
                println!("{}", a[t[quary[1] - 1]][quary[2] - 1])
            }
            _ => (),
        }
    }
}
