use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nc: Vec<&str> = input.split_whitespace().collect();
    let n: usize = nc[0].parse().unwrap();
    let c: Vec<char> = nc[1].chars().collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<char> = input.trim().chars().collect();

    let mut s = 0;
    for i in 0..n {
        if a[i] == 'W' {
            s += 0;
        }
        if a[i] == 'B' {
            s += 1;
        }
        if a[i] == 'R' {
            s += 2
        }
    }

    if s % 3 == 0 && c[0] == 'W' {
        println!("Yes");
    } else if s % 3 == 1 && c[0] == 'B' {
        println!("Yes");
    } else if s % 3 == 2 && c[0] == 'R' {
        println!("Yes");
    } else {
        println!("No");
    }
}
