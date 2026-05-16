fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
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

fn f(a: i64, b: i64, n: i64) -> i64 {
    n * n + a * n + b
}

fn consecutive_prime_size(a: i64, b: i64) -> i64 {
    let mut x = 0;
    while is_prime(f(a, b, x)) {
        x += 1;
    }

    x
}

pub fn solve() -> i64 {
    let mut max_prime_size = 0;
    let mut max_a = 0;
    let mut max_b = 0;

    for a in -999..1000 {
        for b in -1000..1001 {
            let size = consecutive_prime_size(a, b);
            if size > max_prime_size {
                max_prime_size = size;
                (max_a, max_b) = (a, b);
            }
        }
    }   

    max_a * max_b
}
