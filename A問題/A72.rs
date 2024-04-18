use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let hwk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![];
    for _ in 0..hwk[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<char> = input.trim().chars().collect();
        map.push(tmp);
    }

    let mut default = 0;
    for i in 0..hwk[0] {
        for j in 0..hwk[1] {
            if map[i][j] == '#' {
                default += 1;
            }
        }
    }

    let mut ans = 0;
    for x in 0usize..1 << hwk[0] {
        if x.count_ones() as usize > hwk[2] {
            continue;
        }
        let mut tmp = map.clone();
        let mut num = 0;
        for i in 0..hwk[0] {
            if (x >> i & 1) == 1 {
                for j in 0..hwk[1] {
                    if tmp[i][j] == '.' {
                        num += 1;
                        tmp[i][j] = '#';
                    }
                }
            }
        }

        let c = hwk[2] - x.count_ones() as usize;
        if c == 0 {
            ans = ans.max(num);
            continue;
        }

        let mut v = vec![];
        for j in 0..hwk[1] {
            let mut t = 0;
            for i in 0..hwk[0] {
                if tmp[i][j] == '.' {
                    t += 1;
                }
            }
            v.push(t);
        }

        v.sort_by(|x, y| y.cmp(x));
        v.truncate(c);
        ans = ans.max(num + v.into_iter().sum::<usize>());
    }

    println!("{}", ans + default);
}
