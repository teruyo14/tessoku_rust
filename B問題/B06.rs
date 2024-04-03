use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut qsum = vec![0];
    for i in 0..n {
        qsum.push(qsum.last().unwrap() + a[i]);
    }

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let lr: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let tmp = qsum[lr[1]] - qsum[lr[0] - 1];
        let date = lr[1] - lr[0] + 1;
        if tmp * 2 > date {
            println!("win");
        } else if tmp * 2 == date {
            println!("draw");
        } else {
            println!("lose");
        }
    }
}
