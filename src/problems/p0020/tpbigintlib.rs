use num_bigint::BigInt;


fn factorial(n: i64) -> BigInt {
    let mut result = BigInt::from(1);
    for i in 2..=n {
        result *= i;
    }
    result
}

fn sum_of_digits_string(n: &BigInt) -> i64 {
    let mut sum = 0;
    for c in n.to_string().chars() {
        sum += c as i64 - 0x30;
    }
    sum
}

pub fn solve_string() -> i64 {
    let fact = factorial(100);
    sum_of_digits_string(&fact)
}

fn sum_of_digits_math(n: &BigInt) -> i64 {
    let mut sum = 0;
    let ten = BigInt::from(10);
    let one = BigInt::from(1);
    let mut num = n.clone();
    while num > BigInt::from(0) {
        let digit = (&num % &ten) + &one;
        let digit_value = digit.to_u32_digits().1[0] as i64 - 1;
        sum += digit_value;
        num /= &ten;
    }
    sum
}

pub fn solve_math() -> i64 {
    let fact = factorial(100);
    sum_of_digits_math(&fact)
}
