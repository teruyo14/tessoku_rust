use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nwlr: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut x: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let m = 1_000_000_007;
    let mut cum = vec![0 as usize];
    cum.push(1);
    x.insert(0, 0);
    x.push(nwlr[1]);
    for &xi in x.iter().skip(1) {
        if xi < nwlr[2] {
            cum.push(*cum.last().unwrap());
            continue;
        }
        let l_idx = x.partition_point(|&a| a < xi.checked_sub(nwlr[3]).unwrap_or_default());
        let r_idx = x.partition_point(|&a| a <= xi - nwlr[2]);
        cum.push(
            (*cum.last().unwrap()
                + cum[r_idx]
                    .checked_sub(cum[l_idx])
                    .unwrap_or_else(|| cum[r_idx] + m - cum[l_idx]))
                % m,
        );
    }

    let last = *cum.last().unwrap();
    let last2 = cum[cum.len() - 2];
    if last >= last2 {
        println!("{}", last - last2);
    } else {
        println!("{}", last + m - last2);
    }
}
