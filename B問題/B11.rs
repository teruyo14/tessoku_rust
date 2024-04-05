use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: isize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    a.sort();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        let is_ok = |mid: isize| -> bool { a[mid as usize] >= x };
        let ans = meguru_bisect(-1, n, &is_ok);
        println!("{}", ans);
    }
}

fn meguru_bisect<F>(mut ng: isize, mut ok: isize, is_ok: &F) -> isize
where
    F: Fn(isize) -> bool,
{
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
