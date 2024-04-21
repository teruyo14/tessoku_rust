use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nlk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ac = vec![];
    for _ in 0..nlk[0] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ac.push((tmp[0], tmp[1]))
    }

    let mut v = vec![usize::MAX; nlk[1] + 1];
    for &(a, c) in &ac {
        v[a] = min(v[a], c);
    }

    let mut st = SegTree::new(nlk[1] + 1);
    for i in 0..nlk[1] {
        st.update(i, v[i]);
    }

    let m: Vec<_> = (1..=nlk[1] - nlk[2])
        .map(|i| st.query(i, nlk[2] + i))
        .collect();
    if m.iter().any(|&x| x == usize::MAX) {
        println!("-1");
    } else {
        println!("{}", m.iter().sum::<usize>());
    }
}

struct SegTree {
    data: Vec<usize>,
    count: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let count = {
            let mut count = 1;
            while count < n {
                count *= 2;
            }
            count
        };
        let data = vec![usize::MAX; 2 * count - 1];
        Self { data, count }
    }

    fn update(&mut self, p: usize, v: usize) {
        let mut p = p + self.count - 1;
        self.data[p] = v;
        while p > 0 {
            p = (p - 1) / 2;
            self.data[p] = std::cmp::min(self.data[2 * p + 1], self.data[2 * p + 2]);
        }
    }

    fn query(&self, l: usize, r: usize) -> usize {
        self.query_internal(l, r, 0, 0, self.count)
    }

    fn query_internal(&self, l: usize, r: usize, p: usize, a: usize, b: usize) -> usize {
        if l <= a && b <= r {
            return self.data[p];
        }
        if b <= l || r <= a {
            return usize::MAX;
        }
        let m = (a + b) / 2;
        let lv = self.query_internal(l, r, 2 * p + 1, a, m);
        let rv = self.query_internal(l, r, 2 * p + 2, m, b);
        min(lv, rv)
    }
}
