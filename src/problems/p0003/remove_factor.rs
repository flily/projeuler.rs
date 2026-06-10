const NUMBER: i64 = 600851475143;

fn is_prime(n: i64) -> bool {
    if n <= 2 {
        return true;
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

fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

pub fn solve() -> i64 {
    let n = NUMBER;

    let mut i = 3;
    let mut last = 3;
    let mut m = n;

    while m > 0 && i <= m {
        if m % i == 0 && is_prime(i) {
            last = i;
            m = remove_factor(m, i);
        }

        i += 2;
    }

    last
}
