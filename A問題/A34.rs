use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nxy: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut grundy = vec![0; 100010];
    for i in 0..100010 {
        let mut transit = [false; 3];
        if i >= nxy[1] {
            transit[grundy[i - nxy[1]]] = true;
        }

        if i >= nxy[2] {
            transit[grundy[i - nxy[2]]] = true;
        }

        if transit[0] == false {
            grundy[i] = 0;
        } else if transit[1] == false {
            grundy[i] = 1;
        } else {
            grundy[i] = 2;
        }
    }

    let mut cnt = 0;
    for i in 0..nxy[0] {
        cnt ^= grundy[a[i]];
    }

    if cnt >= 1 {
        println!("First");
    } else {
        println!("Second");
    }
}
