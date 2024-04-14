use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let a: usize = input.trim().parse().unwrap();
        *map.entry(a).or_insert(0) += 1;
        set.insert(a);
    }

    let mut ans: usize = 0;
    for i in set {
        if map[&i] >= 2 {
            ans += map[&i] * (map[&i] - 1) / 2;
        }
    }

    println!("{}", ans);
}
