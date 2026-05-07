fn sieve_primes_sum(size: usize) -> i64 {
    let mut sieve = vec![true; size / 2];
    let mut n = 3;
    let mut sum: i64 = 2;

    while n < size {
        if sieve[n / 2] {
            let mut m = n * n;
            while m < size{
                sieve[m / 2] = false;
                m += n * 2;
            }
            sum += n as i64;
        }

        n += 2;
    }

    sum
}

pub fn solve() -> i64 {
    sieve_primes_sum(2_000_000)
}
