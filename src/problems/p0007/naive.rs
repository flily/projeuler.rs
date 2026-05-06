fn is_prime(n: i64) -> bool {
    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

static LIMIT: i64 = 10001;


pub fn solve() -> i64 {
    let mut count = 1;
    let mut n = 3;

    loop {
        if is_prime(n) {
            count += 1;
            if count >= LIMIT {
                return n;
            }
        }

        n += 2;
    }
}