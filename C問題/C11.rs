use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ok = 1_000_000_100.;
    let mut ng = 0.;
    while ok - ng > 0.000001 {
        let m = (ok + ng) / 2.;
        let seat = a.iter().map(|f| (f / m) as usize).sum::<usize>();
        if seat > nk[1] {
            ng = m;
        } else {
            ok = m;
        }
    }
    let ans = a.iter().map(|f| (f / ok) as usize).collect::<Vec<_>>();
    for i in ans {
        println!("{} ", i);
    }
    println!("")
}
