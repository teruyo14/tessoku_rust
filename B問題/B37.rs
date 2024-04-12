use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let mut power10 = vec![];
    for i in 0..17 {
        power10.push(10u64.pow(i as u32));
    }

    let mut r = vec![vec![0; 10]; 17];
    for i in 0..16 {
        let digit = (n / power10[i]) % 10;

        for j in 0..10 {
            if j < digit {
                r[i][j as usize] = (n / power10[i + 1] + 1) * power10[i]
            }
            if j == digit {
                r[i][j as usize] = (n / power10[i + 1]) * power10[i] + (n % power10[i]) + 1;
            }
            if j > digit {
                r[i][j as usize] = (n / power10[i + 1]) * power10[i]
            }
        }
    }

    let mut ans = 0;
    for i in 0..16 {
        for j in 0..10 {
            ans += j * r[i][j] as usize;
        }
    }

    println!("{}", ans);
}
