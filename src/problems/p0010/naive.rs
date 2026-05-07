fn is_prime(n: i64) -> bool {
    // assume n > 3 and n is odd
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

static LIMIT: i64 = 2_000_000;

pub fn solve() -> i64 {
    let mut sum = 2;
    
    for n in (3..LIMIT).step_by(2) {
        if is_prime(n) {
            sum += n;
        }
    }

    sum
}
