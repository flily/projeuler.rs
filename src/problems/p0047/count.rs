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

fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

fn find_prime_factors(n: i64) -> i64{
    let mut count = 0;
    let mut m = n;
    if m % 2 == 0 {
        count += 1;
        m = remove_factor(m, 2);
    }

    let mut i = 3;
    while i * i <= n {
        if m % i == 0 &&  is_prime(i) {
            count += 1;
            m = remove_factor(m, i);
        }
        i += 2;
    }

    if m > 1 && is_prime(m) {
        count += 1;
    }

    count
}

pub fn solve() -> i64 {
    let mut n = 647;
    let mut count = 0;

    loop {
        let c = find_prime_factors(n);
        if c == 4 {
            count += 1;
        } else {
            count = 0;
        }

        if count == 4 {
            break;
        }

        n += 1;
    }

    n - 3
}
