use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let k: usize = input.trim().parse().unwrap();

    let mut lr = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lr.push(tmp);
    }

    let lr2 = lr.clone();
    lr.sort_by_key(|x| x[1]);
    let mut before = vec![0; 86401];
    let mut tmp = 0;
    for i in 0..n {
        if tmp <= lr[i][0] {
            tmp = lr[i][1] + k;
            if tmp < 86401 {
                before[tmp] += 1;
            }
        }
    }

    for i in 1..86401 {
        before[i] += before[i - 1];
    }

    lr.sort_by_key(|x| x[0]);
    let mut after = vec![0; 86401];
    tmp = 86401;
    for i in (0..n).rev() {
        if tmp >= lr[i][1] {
            after[lr[i][0]] += 1;
            if lr[i][0] >= k {
                tmp = lr[i][0] - k;
            } else {
                break;
            }
        }
    }

    for i in (0..86400).rev() {
        after[i] += after[i + 1];
    }

    for i in 0..n {
        let mut ans = 0;
        ans += before[lr2[i][0]];
        if lr2[i][1] + k < 86401 {
            ans += after[lr2[i][1] + k];
        }
        ans += 1;
        println!("{}", ans);
    }
}
