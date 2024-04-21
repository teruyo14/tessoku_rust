use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut que1 = VecDeque::new();
    let mut que2 = VecDeque::new();
    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<&str> = input.split_whitespace().collect();
        match query[0] {
            "A" => {
                que2.push_back(query[1].to_string());
            }
            "B" => {
                que2.push_front(query[1].to_string());
            }
            "C" => {
                que1.pop_front();
            }
            "D" => {
                println!("{}", que1.front().unwrap());
            }
            _ => (),
        }
        if que1.len() < que2.len() {
            que1.push_back(que2.pop_front().unwrap());
        }
    }
}
