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
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ok = 10isize.pow(9) + 10;
    let mut ng = -1;
    let is_ok = |mid: isize| -> bool {
        let mut cnt = 0;
        for i in 0..nk[0] {
            cnt += (mid as usize) / a[i];
        }
        cnt >= nk[1]
    };

    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
