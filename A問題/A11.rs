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

    let is_ok = |mid: usize| -> bool { a[mid] >= nx[1] };
    let ans = meguru_bisect(0usize.wrapping_sub(1), a.len(), &is_ok);

    println!("{}", ans + 1);
}

fn meguru_bisect<F>(mut ng: usize, mut ok: usize, is_ok: &F) -> usize
where
    F: Fn(usize) -> bool,
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
