fn is_pandigit(n: i64) -> bool {
    let mut digits = [false; 10];
    let size = (n as f64).log10().floor() as usize + 1;
    let mut m = n;

    while m > 0 {
        let d = (m % 10) as usize;
        if d == 0 || digits[d] || d > size {
            return false;
        }

        digits[d] = true;
        m /= 10;
    }

    true
}

fn is_prime(n: i64) -> bool {
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

pub fn solve() -> i64 {
    let mut n = 987654321;
    while n > 0 {
        if is_pandigit(n) && is_prime(n) {
            break;
        }

        n -= 2;
    }

    n
}
