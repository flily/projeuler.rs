use malachite::Integer;

fn factorial(n: i64) -> Integer {
    let mut result = Integer::from(1);
    for i in 2..=n {
        result *= Integer::from(i);
    }
    result
}

fn sum_of_digits_string(n: &Integer) -> i64 {
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
