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

pub fn solve() -> i64 {
    let n = NUMBER;

    let mut i = 3;
    let mut last = 3;

    while 2 * i <= n {
        if n % i == 0 && is_prime(i) {
            last = i;
        }

        i += 2;
    }

    last
}
