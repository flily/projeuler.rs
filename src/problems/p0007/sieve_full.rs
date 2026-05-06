fn sieve_primes(max_prime: usize, limit: i64) -> i64 {
    let mut sieve = vec![true; max_prime];
    let mut count = 1;
    let mut n = 3;

    while count < limit {
        if sieve[n as usize] {
            count += 1;
            if count >= limit {
                return n;
            }

            let mut m = n * n;
            while m < max_prime as i64 {
                sieve[m as usize] = false;
                m += n * 2;
            }
        }

        n += 2;
    }

    -1
}

pub fn solve() -> i64 {
    sieve_primes(120000, 10001)
}