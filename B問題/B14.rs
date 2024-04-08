use std::io::stdin;
use std::process::exit;

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

    let half = nk[0] / 2;
    let mut first_half = vec![];
    let mut second_half = vec![];

    for i in 0..1 << half {
        let mut sum = 0;
        for j in 0..half {
            if i & (1 << j) != 0 {
                sum += a[j];
            }
        }
        first_half.push(sum);
    }

    for i in 0..1 << (nk[0] - half) {
        let mut sum = 0;
        for j in 0..nk[0] - half {
            if i & (1 << j) != 0 {
                sum += a[half + j];
            }
        }
        second_half.push(sum);
    }

    second_half.sort();
    for sum in first_half.iter() {
        if second_half.binary_search(&(nk[1] - sum)).is_ok() {
            println!("Yes");
            exit(0);
        }
    }

    println!("No");
}
