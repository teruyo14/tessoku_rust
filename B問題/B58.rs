use std::cmp::min;
use std::io::stdin;

struct SegmentTree<T> {
    size: usize,
    tree: Vec<T>,
    choose: fn(T, T) -> T,
    initial_value: T,
}

impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, initial_value: T, choose: fn(T, T) -> T) -> SegmentTree<T> {
        let size = n.next_power_of_two();
        let tree = vec![initial_value; 2 * size];
        SegmentTree {
            size,
            tree,
            choose,
            initial_value,
        }
    }

    fn update(&mut self, pos: usize, x: T) {
        fn update_upper<T: Copy>(i: usize, segment_tree: &mut SegmentTree<T>) {
            if i >= 1 {
                let choose = segment_tree.choose;
                segment_tree.tree[i] =
                    choose(segment_tree.tree[2 * i], segment_tree.tree[2 * i + 1]);
                update_upper(i / 2, segment_tree);
            }
        }
        let i = pos + self.size - 1;
        self.tree[i] = x;
        update_upper(i / 2, self);
    }

    fn get_chosen(&self, l: usize, r: usize) -> T {
        fn get<T: Copy>(
            l: usize,
            r: usize,
            i: usize,
            left: usize,
            right: usize,
            segment_tree: &SegmentTree<T>,
        ) -> T {
            if l <= left && right <= r {
                segment_tree.tree[i]
            } else if right <= l || r <= left {
                segment_tree.initial_value
            } else {
                let choose = segment_tree.choose;
                choose(
                    get(l, r, 2 * i, left, (left + right) / 2, segment_tree),
                    get(l, r, 2 * i + 1, (left + right) / 2, right, segment_tree),
                )
            }
        }
        get(l, r, 1, 1, self.size + 1, self)
    }

    fn get(&self, pos: usize) -> T {
        self.tree[pos + self.size - 1]
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nlr: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let x: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut segtree = SegmentTree::new(nlr[0], usize::MAX, min);
    segtree.update(1, 0);

    for (i, &xi) in x.iter().enumerate().skip(1) {
        let i = i + 1;
        let left = if xi <= nlr[1] {
            0
        } else {
            x.partition_point(|&x| x + nlr[2] < xi)
        } + 1;
        let right = if xi < nlr[2] {
            1
        } else {
            x.partition_point(|&x| x + nlr[1] <= xi)
        } + 1;
        let a = segtree.get_chosen(left, right).saturating_add(1);
        segtree.update(i, a);
    }
    let ans = segtree.get(nlr[0]);
    println!("{}", ans);
}
