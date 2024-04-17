use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = vec![vec![]; nm[0] + 1];
    for _ in 0..nm[1] {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let abc: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map[abc[0] - 1].push((abc[1] - 1, abc[2]));
        map[abc[1] - 1].push((abc[0] - 1, abc[2]));
    }

    let mut confirm = vec![-1; nm[0]];

    let mut cost = vec![usize::MAX; nm[0]];
    let mut hqueue = BinaryHeap::new();

    push(-1, nm[0] - 1, 0, &mut cost, &mut confirm, &mut hqueue);
    while let Some(Reverse((c, x))) = hqueue.pop() {
        if cost[x] != c {
            continue;
        }
        for (j, d) in &map[x] {
            push(
                x as isize,
                (*j as isize).try_into().unwrap(),
                c + d,
                &mut cost,
                &mut confirm,
                &mut hqueue,
            )
        }
    }

    let mut ans = vec![0];
    while *ans.last().unwrap() as usize != nm[0] - 1 {
        ans.push(confirm[*ans.last().unwrap() as usize]);
    }

    for i in ans {
        print!("{} ", i + 1);
    }
    println!("");
}

fn push(
    prev: isize,
    i: usize,
    c: usize,
    cost: &mut Vec<usize>,
    confirm: &mut Vec<isize>,
    hqueue: &mut BinaryHeap<Reverse<(usize, usize)>>,
) {
    if cost[i] <= c {
        return;
    }
    cost[i] = c;
    confirm[i] = prev;
    hqueue.push(Reverse((c, i)));
}
