use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut dp: Vec<usize> = vec![0; 10_000_010];
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..10_000_010 {
        dp[i] = (dp[i - 1] + dp[i - 2]) % (1000_000_000 + 7);
    }

    println!("{}", dp[n]);
}
