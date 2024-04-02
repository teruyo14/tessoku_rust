use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let from_base = 2;
    let to_base = 10;
    let ans = radix_conversion(input, from_base, to_base).unwrap();
    println!("{:}", ans);
}

fn radix_conversion(input: &str, a: u32, b: u32) -> Result<String, &'static str> {
    let num_in_dec = u64::from_str_radix(input, a).unwrap();

    let mut result = Vec::new();
    let mut quotient = num_in_dec;

    if quotient == 0 {
        return Ok("0".to_string());
    }

    while quotient > 0 {
        let remainder = quotient % b as u64;
        quotient /= b as u64;
        result.push(std::char::from_digit(remainder as u32, b).unwrap());
    }

    result.reverse();
    Ok(result.into_iter().collect())
}
