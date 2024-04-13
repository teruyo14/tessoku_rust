use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut ab = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<isize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ab.push(tmp);
    }

    let ans1 = solve(true, true, &ab, n);
    let ans2 = solve(true, false, &ab, n);
    let ans3 = solve(false, true, &ab, n);
    let ans4 = solve(false, false, &ab, n);

    println!("{}", max(max(max(ans1, ans2), ans3), ans4))
}

fn solve(front: bool, back: bool, l: &Vec<Vec<isize>>, n: usize) -> isize {
    let mut cnt = 0;
    for i in 0..n {
        let mut card1 = l[i][0];
        if front == false {
            card1 = -l[i][0];
        }
        let mut card2 = l[i][1];
        if back == false {
            card2 = -l[i][1];
        }

        if card1 + card2 >= 0 {
            cnt += card1 + card2;
        }
    }
    cnt
}
