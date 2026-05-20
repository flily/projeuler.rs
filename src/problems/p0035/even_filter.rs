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

fn find_circular_primes(digits: &[i64], size: usize, index: usize, num: i64) -> i64 {
    let mut count = 0;

    if index == size {
        if is_circular_prime(num) {
            count += 1;
        }
    } else {
        for &d in digits {
            let new_num = num * 10 + d;
            count += find_circular_primes(digits, size, index + 1, new_num);
        }
    }

    count
}

pub fn solve() -> i64 {
    let digits = [1, 3, 5, 7, 9];
    let mut count = 13;
    for size in 3..=6 {
        count += find_circular_primes(&digits, size, 0, 0);
    }

    count
}
