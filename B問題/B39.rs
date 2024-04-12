use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nd: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut xy = vec![];
    for _ in 0..nd[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        xy.push(tmp);
    }

    let mut used = vec![false; nd[0]];

    let mut ans = 0;
    for i in 1..=nd[1] {
        let mut v = 0;
        let mut id = 1_000_000_000;
        for j in 0..nd[0] {
            if used[j] == false && v < xy[j][1] && xy[j][0] <= i {
                v = xy[j][1];
                id = j;
            }
        }

        if id != 1_000_000_000 {
            ans += v;
            used[id] = true;
        }
    }

    println!("{}", ans);
}
