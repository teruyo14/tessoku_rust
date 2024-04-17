use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut abc = vec![];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        abc.push(tmp)
    }

    abc.sort_by_key(|f| f[2]);
    
    let mut uf = UnionFind::new(nm[0]);
    let mut ans = 0;
    for t in abc {
        if !uf.same(t[0] - 1, t[1] - 1) {
            uf.unite(t[0] - 1, t[1] - 1);
            ans += t[2];
        }
    }
    
    println!("{}", ans);
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
