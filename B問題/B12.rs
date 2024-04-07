use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: f64 = input.trim().parse().unwrap();

    let mut ok = 10.0f64.powf(5.0) + 10.;
    let mut ng = -1.;
    let is_ok = |mid: f64| -> bool {
        n - mid.powf(3.0) -mid <= 0.
    };

    while ok - ng > 0.0001 {
        let mid = ng + (ok - ng) / 2.;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
