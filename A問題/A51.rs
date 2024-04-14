use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut stack = vec![];
    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let quary: Vec<&str> = input.split_whitespace().collect();
        match quary[0] {
            "1" => stack.push(quary[1].to_string()),
            "2" => {
                if let Some(top) = stack.last() {
                    println!("{}", top);
                }
            }
            "3" => {
                stack.pop();
            }
            _ => (),
        }
    }
}
