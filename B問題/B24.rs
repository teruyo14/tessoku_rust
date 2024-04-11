use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut boxes = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let xy: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        boxes.push((xy[0], xy[1]));
    }

    boxes.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut l = vec![];
    for &(_, y) in &boxes {
        match l.binary_search(&y) {
            Ok(_) => (),
            Err(pos) => {
                if pos == l.len() {
                    l.push(y);
                } else {
                    l[pos] = y;
                }
            }
        }
    }

    println!("{}", l.len());
}
