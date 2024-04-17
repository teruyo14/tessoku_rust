use std::io::stdin;

enum Query {
    One(usize),
    Two(usize, usize),
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ab = vec![];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ab.push(tmp);
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let q = input.trim().parse().unwrap();

    let mut query = vec![];
    let mut remove = vec![false; nm[0]];
    for _ in 0..q {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tmp: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match tmp[0] {
            1 => {
                remove[tmp[1] - 1] = true;
                query.push(Query::One(tmp[1] - 1));
            }
            2 => query.push(Query::Two(tmp[1], tmp[2])),
            _ => (),
        }
    }

    query.reverse();

    let mut uf = UnionFind::new(nm[0]);
    for i in 0..nm[1] {
        if !remove[i] {
            uf.unite(ab[i][0] - 1, ab[i][1] - 1);
        }
    }

    let mut ans = vec![];
    for que in query {
        match que {
            Query::One(x) => {
                uf.unite(ab[x][0] - 1, ab[x][1] - 1);
            }
            Query::Two(u, v) => {
                ans.push(if uf.same(u - 1, v - 1) { "Yes" } else { "No" });
            }
        }
    }

    ans.reverse();
    for i in ans.into_iter() {
        println!("{}", i);
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
