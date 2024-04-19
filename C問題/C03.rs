use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let d: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let x: usize = input.trim().parse().unwrap();

    let mut price = vec![x; d];
    for td in 1..d {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let a: i128 = input.trim().parse().unwrap();

        price[td] = (price[td - 1] as i128 + a) as usize;
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let st: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if price[st[0] - 1] == price[st[1] - 1] {
            println!("Same");
        } else if price[st[0] - 1] > price[st[1] - 1] {
            println!("{}", st[0]);
        } else {
            println!("{}", st[1]);
        }
    }
}
