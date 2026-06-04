fn is_prime(n: i64) -> bool {
    // n is odd and n > 2

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

fn is_integer_double_square(n: i64) -> bool {
    if n % 2 != 0 {
        false
    } else {
        let h = n / 2;
        let s = (h as f64).sqrt() as i64;
        s * s == h
    }
}

fn is_goldbach(n: i64) -> bool {
    if is_integer_double_square(n - 2) {
        return true;
    }

    let mut i = 3;
    while i <= n - 2 {
        if is_prime(i) && is_integer_double_square(n - i) {
            return true;
        }

        i += 2;
    }

    false
}

pub fn solve() -> i64 {
    let mut result = 9;
    loop {
        if !is_prime(result) && !is_goldbach(result) {
            break
        }

        result += 2;
    }

    result
}
