use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut st = vec![];
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        st.push(tmp);
    }

    let mut tf = vec![true; 10000];
    let mut ans = vec![];
    for i in 0..10000 {
        for j in 0..n {
            let mut p = 1;
            let mut cnt = 0;
            for _ in 0..4 {
                if i / p % 10 == st[j][0] / p % 10 {
                    cnt += 1;
                }
                p *= 10;
            }
            match st[j][1] {
                1 => tf[i] &= cnt == 4,
                2 => tf[i] &= cnt == 3,
                3 => tf[i] &= cnt < 3,
                _ => (),
            }
        }
        if tf[i] {
            ans.push(i);
        }
    }

    if ans.len() == 1 {
        println!("{:04}", ans[0]);
    } else {
        println!("Can't Solve");
    }
}
