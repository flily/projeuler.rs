fn is_prime(n: i64) -> bool {
    // assume n > 2
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn is_circular_prime(n: i64) -> bool {
    if !is_prime(n) {
        return false;
    }

    let mut m = n;
    let size = ((n as f64).log10().floor() as i64) + 1;
    for _ in 0..size {
        let last = m % 10;
        m = (m / 10) + last * (10_i64.pow(size as u32 - 1));
        if !is_prime(m) {
            return false;
        }
    }

    true
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut count = 13;

    for n in (101..LIMIT).step_by(2) {
        if is_circular_prime(n) {
            count += 1;
        }
    }

    count
}
