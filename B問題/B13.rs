use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nk: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut psum = vec![0];
    for i in 0..nk[0] {
        psum.push(psum.last().unwrap() + a[i]);
    }

    let mut ans = 0;
    for i in 0..nk[0] {
        let mut low = i;
        let mut high = nk[0] + 1;
        while high - low > 1 {
            let mid = (low + high) / 2;
            if psum[mid] - psum[i] <= nk[1] {
                low = mid;
            } else {
                high = mid;
            }
        }
        ans += low - i;
    }

    println!("{}", ans);
}
