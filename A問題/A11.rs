use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nx: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<isize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let is_ok = |mid: isize| -> bool { a[mid as usize] >= nx[1] };
    let ans = meguru_bisect(-1, nx[0], &is_ok);

    println!("{}", ans + 1);
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
