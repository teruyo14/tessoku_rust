use std::io::stdin;
use std::process::exit;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let mut cnt_b = 0;
    let mut cnt_r = 0;
    for ch in s.chars() {
        if ch == 'B' {
            cnt_b += 1;
            cnt_r = 0;
            if cnt_b == 3 {
                println!("Yes");
                exit(0);
            }
        } else if ch == 'R' {
            cnt_b = 0;
            cnt_r += 1;
            if cnt_r == 3 {
                println!("Yes");
                exit(0);
            }
        } else {
            cnt_b = 0;
            cnt_r = 0;
        }
    }

    println!("No");
}
