use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let s: Vec<char> = input.trim().chars().collect();

    let mut stack = vec![];
    for i in 0..s.len() {
        match s[i] {
            '(' => stack.push(i + 1),
            ')' => {
                if let Some(v) = stack.pop() {
                    println!("{} {}", v, i + 1);
                }
            }
            _ => (),
        }
    }
}
