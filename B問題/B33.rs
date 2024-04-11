use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nhw: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ab = vec![];
    for _ in 0..nhw[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ab.push(tmp);
    }

    let mut cnt = 0;
    for i in 0..nhw[0] {
        cnt ^= ab[i][0] - 1;
        cnt ^= ab[i][1] - 1;
    }

    if cnt != 0 {
        println!("First");
    } else {
        println!("Second")
    }
}
