use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut map = HashMap::new();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<&str> = input.split_whitespace().collect();
        match query[0] {
            "1" => {
                let score: usize = query[2].parse().unwrap();
                map.insert(query[1].to_string(), score);
            }
            "2" => {
                if let Some(v) = map.get(query[1]) {
                    println!("{}", v);
                }
            }
            _ => (),
        }
    }
}
