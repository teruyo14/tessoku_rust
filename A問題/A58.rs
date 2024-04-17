use std::cmp::max;
use std::io::stdin;

struct SegTree {
    size: usize,
    dat: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> SegTree {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        SegTree {
            size,
            dat: vec![0; size * 2],
        }
    }

    fn update(&mut self, mut pos: usize, x: i32) {
        pos += self.size;
        self.dat[pos] = x;
        while pos >= 2 {
            pos /= 2;
            self.dat[pos] = max(self.dat[pos * 2], self.dat[pos * 2 + 1]);
        }
    }

    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i32 {
        if r <= a || b <= l {
            -1_000_000_000
        } else if l <= a && b <= r {
            self.dat[u]
        } else {
            let m = (a + b) / 2;
            let ans_l = self.query(l, r, a, m, u * 2);
            let ans_r = self.query(l, r, m, b, u * 2 + 1);
            std::cmp::max(ans_l, ans_r)
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut segtree = SegTree::new(nq[0]);

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let q: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match q[0] {
            1 => {
                segtree.update(q[1] - 1, q[2] as i32);
            }
            2 => {
                let ans = segtree.query(q[1] - 1, q[2] - 1, 0, segtree.size, 1);
                println!("{}", ans);
            }
            _ => (),
        }
    }
}
