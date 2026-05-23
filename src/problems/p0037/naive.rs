fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt().ceil() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn is_truncatable_prime_left(n: i64) -> bool {
    let mut m = n;
    while m > 0 {
        if !is_prime(m) {
            return false;
        }

        let size = (m as f64).log10().floor() as u32 + 1;
        m %= 10_i64.pow(size - 1);
    }

    true
}

fn is_truncatable_prime_right(n: i64) -> bool {
    let mut m = n;
    while m > 0 {
        if !is_prime(m) {
            return false;
        }

        m /= 10;
    }

    true
}

pub fn solve() -> i64 {
    let mut sum = 0;
    let mut count = 0;

    let mut n = 11;
    while count < 11 {
        if is_truncatable_prime_left(n) && is_truncatable_prime_right(n) {
            sum += n;
            count += 1;
        }

        n += 2;
    }

    sum
}
