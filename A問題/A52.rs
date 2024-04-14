use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let quary: Vec<&str> = input.split_whitespace().collect();
        match quary[0] {
            "1" => queue.push_back(quary[1].to_string()),
            "2" => {
                println!("{}",queue[0]);
            }
            "3" => {
                queue.pop_front();
            }
            _ => (),
        }
    }
}
