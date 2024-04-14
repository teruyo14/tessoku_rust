use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match query[0] {
            1 => heap.push(Reverse(query[1])),
            2 => {
                if let Some(Reverse(v)) = heap.peek() {
                    println!("{}", v);
                }
            }
            3 => {
                if let Some(Reverse(_v)) = heap.pop() {
                    ()
                }
            }
            _ => (),
        }
    }
}
