fn factorial(n: i64) -> i64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    result
}

fn is_sum_of_digital_factorials(n: i64) -> bool {
    let mut sum = 0;
    let mut m = n;

    while m > 0 {
        sum += factorial(m % 10);
        m /= 10;
    }

    sum == n
}

pub fn solve() -> i64 {
    let mut sum = 0;
    let max_n = factorial(9) + factorial(8);
    for n in 3..=max_n {
        if is_sum_of_digital_factorials(n) {
            sum += n;
        }
    }

    sum
}
