use std::cmp::PartialOrd;
use std::io::stdin;
use std::ops::{Div, Rem};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();
    let ans = radix_conversion(n, 2); // 基数変換のみ必要なので、2進数への変換のみを指定
    println!("{:0>10}", ans);
}

fn radix_conversion<T>(mut n: T, b: T) -> String
where
    T: Div<Output = T> + Rem<Output = T> + From<u8> + Copy + PartialOrd + ToString,
{
    let mut x = String::new();
    while n > T::from(0u8) {
        let tmp = n % b;
        n = n / b;
        x.push(
            std::char::from_digit(tmp.to_string().parse::<u32>().unwrap(), 10)
                .expect("Should be a digit"),
        );
    }
    if x.is_empty() {
        x.push('0');
    }
    x.chars().rev().collect()
}
