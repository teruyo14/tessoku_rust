use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let p: Vec<isize> = input
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
        ab.push((tmp[0], tmp[1]))
    }

    let mut ans = 0;
    let mut abc = vec![];
    for (pi, idx) in p.into_iter().zip(1..) {
        if pi >= 0 {
            abc.push((0, idx, pi as usize));
            abc.push((idx, nm[0] + 1, 0));
            ans += pi;
        } else {
            abc.push((0, idx, 0));
            abc.push((idx, nm[0] + 1, (-pi) as usize));
        }
    }

    for (a, b) in ab.into_iter() {
        abc.push((a, b, usize::MAX));
    }

    let t = max_flow(abc, nm[0] + 1, 0, nm[0] + 1);
    println!("{}", ans - t as isize);
}

fn max_flow(abc: Vec<(usize, usize, usize)>, n: usize, start: usize, goal: usize) -> usize {
    fn dfs(
        al: &mut Vec<Vec<(usize, usize, usize)>>,
        f: usize,
        pos: usize,
        goal: usize,
        visited: &mut Vec<bool>,
    ) -> usize {
        if pos == goal {
            return f;
        }
        visited[pos] = true;
        for idx in 0..al[pos].len() {
            let (next, capa, rev) = al[pos][idx];
            if capa == 0 || visited[next] {
                continue;
            }
            let flow = dfs(al, f.min(capa), next, goal, visited);
            if flow != 0 {
                al[pos][idx].1 -= flow;
                al[next][rev].1 += flow;
                return flow;
            }
        }
        return 0;
    }
    let mut al = vec![vec![]; n + 1];
    for (a, b, c) in abc.into_iter() {
        let a_len = al[a].len();
        let b_len = al[b].len();
        al[a].push((b, c, b_len));
        al[b].push((a, 0, a_len));
    }
    let mut ret = 0;
    loop {
        let mut visited = vec![false; n + 1];
        let f = dfs(&mut al, usize::MAX, start, goal, &mut visited);
        if f == 0 {
            break;
        }
        ret += f;
    }
    ret
}
