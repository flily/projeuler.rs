const NUMBER: i64 = 600851475143;

fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

fn find_largest_prime_factor(n: i64) -> i64 {
    let mut sqrt = (n as f64).sqrt() as i64;
    let mut sieve = vec![true; (sqrt + 1) as usize];
    sieve[0] = false;

    let mut m = n;
    let mut p = 3;
    let mut max_prime_factor = 3;
    while p * p < m {
        if sieve[(p / 2) as usize] {
            if m % p == 0 {
                max_prime_factor = p;
                sqrt = (m as f64).sqrt() as i64;
                m = remove_factor(m, p);
            }

            for i in (p * p..=sqrt).step_by(p as usize) {
                sieve[(i / 2) as usize] = false;
            }
        }

        p += 2;
    }

    max_prime_factor
}

pub fn solve() -> i64 {
    find_largest_prime_factor(NUMBER)
}
