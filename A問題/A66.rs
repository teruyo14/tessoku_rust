use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut uf = UnionFind::new(nq[0]);

    for _ in 0..nq[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match query[0] {
            1 => uf.unite(query[1] - 1, query[2] - 1),
            2 => {
                if uf.same(query[1] - 1, query[2] - 1) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => (),
        }
    }
}

struct UnionFind {
    par: Vec<isize>,
    siz: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            siz: vec![1; n],
        }
    }

    pub fn root(&self, x: usize) -> usize {
        let mut pos = x;
        loop {
            if self.par[pos] == -1 {
                break;
            }
            pos = self.par[pos] as usize;
        }
        return pos;
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_y == root_x {
            return;
        }
        if self.siz[root_x] > self.siz[root_y] {
            self.par[root_y] = root_x as isize;
            self.siz[root_x] += self.siz[root_y]
        } else {
            self.par[root_x] = root_y as isize;
            self.siz[root_y] += self.siz[root_x]
        }
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        if self.root(x) == self.root(y) {
            return true;
        } else {
            return false;
        }
    }
}
