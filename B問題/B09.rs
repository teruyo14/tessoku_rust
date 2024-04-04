use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    let mut imos = vec![vec![0; 1502]; 1502];

    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let abcd: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        imos[abcd[1]][abcd[0]] += 1;
        imos[abcd[1]][abcd[2]] -= 1;
        imos[abcd[3]][abcd[0]] -= 1;
        imos[abcd[3]][abcd[2]] += 1;
    }

    let mut psum = vec![vec![0; 1502]; 1502];
    for i in 1..=1500 {
        for j in 1..=1500 {
            psum[i][j] = psum[i - 1][j] + psum[i][j - 1] - psum[i - 1][j - 1] + imos[i - 1][j - 1];
        }
    }

    let mut ans = 0;
    for i in 1..=1500 {
        for j in 1..=1500 {
            if psum[i][j] >= 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
