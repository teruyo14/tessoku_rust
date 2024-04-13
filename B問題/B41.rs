use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let xy: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut x = xy[0];
    let mut y = xy[1];

    let mut ans = vec![];
    while x >= 2 || y >= 2 {
        ans.push([x, y]);
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    ans.reverse();
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i][0], ans[i][1]);
    }
}
