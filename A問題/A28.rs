use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut ans = 0;
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let ta: Vec<&str> = input.split_whitespace().collect();
        let t = ta[0].chars().next().unwrap();
        let a: isize = ta[1].parse().unwrap();

        match t {
            '+' => ans += a,
            '-' => ans -= a,
            '*' => ans *= a,
            _ => {}
        }

        ans %= 10000;
        if ans < 0 {
            ans += 10000;
        }
        println!("{}", ans);
    }
}
