use num_bigint::BigInt;

pub fn solve_integer() -> i64 {
    let mut num = BigInt::from(2).pow(1000);
    let mut sum = 0;
    let ten = BigInt::from(10);
    let zero = BigInt::from(0);
    let one = BigInt::from(1);

    while num > zero {
        let digit = (&num % &ten) + &one;
        sum += digit.to_u32_digits().1[0] as i64 - 1;
        num /= &ten;
    }

    sum
}

pub fn solve_string() -> i64 {
    let num = BigInt::from(2).pow(1000);
    num.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .sum()
}
