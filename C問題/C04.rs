use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut ans = vec![];
    for i in 1..=n {
        if i * i > n {
            break;
        }

        if n % i != 0 {
            continue;
        }

        ans.push(i);
        if n / i != i {
            ans.push(n / i);
        }
    }

    ans.sort();
    for i in ans {
        println!("{}", i);
    }
}
