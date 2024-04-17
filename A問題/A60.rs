use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ans = vec![-1; n];
    let mut l: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..n {
        if i >= 1 {
            l.push_back((i, a[i - 1]));
            while let Some(&(_, kabuka)) = l.back() {
                if kabuka <= a[i] {
                    l.pop_back();
                } else {
                    break;
                }
            }
        }
        if let Some(&(date, _)) = l.back() {
            ans[i] = date as i32;
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
