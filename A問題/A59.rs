use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut segtree = SegmentTree::new(nq[0], |a, b| a + b, 0);

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                segtree.set(query[1] - 1, query[2] as i32);
            }
            2 => {
                println!("{}", segtree.prod(query[1] - 1, query[2] - 1));
            }
            _ => (),
        }
    }
}

struct SegmentTree<T> {
    tree: Vec<T>,
    size: usize,
    op: fn(T, T) -> T,
    e: T,
}

impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self {
        let size = n.next_power_of_two();
        Self {
            tree: vec![e; 2 * size],
            size,
            op,
            e,
        }
    }

    fn set(&mut self, mut k: usize, x: T) {
        k += self.size;
        self.tree[k] = x;
        while k > 0 {
            k /= 2;
            self.tree[k] = (self.op)(self.tree[2 * k], self.tree[2 * k + 1]);
        }
    }

    fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.size;
        r += self.size;
        let mut res = self.e;
        while l < r {
            if l % 2 == 1 {
                res = (self.op)(res, self.tree[l]);
                l += 1;
            }
            l /= 2;
            if r % 2 == 1 {
                res = (self.op)(res, self.tree[r - 1]);
                r -= 1;
            }
            r /= 2;
        }
        res
    }
}
