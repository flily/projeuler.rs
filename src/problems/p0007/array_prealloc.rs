static LIMIT: usize = 10001;

pub fn solve() -> i64 {
    let mut primes = vec![0; LIMIT];
    primes[0] = 3;
    primes[1] = 5;
    primes[2] = 7;
    primes[3] = 11;
    primes[4] = 13;
    primes[5] = 17;
    primes[6] = 19;

    let mut i = 7;

    for n in (23..).step_by(2) {
        let mut is_prime = true;
        for j in 0..i {
            let p = primes[j];
            if n % p == 0 {
                is_prime = false;
                break;
            }

            if p * p > n {
                break;
            }
        }

        if is_prime {
            primes[i] = n;
            i += 1;
        }

        if i >= LIMIT - 1 {
            // 2 is not counted
            break;
        }
    }

    primes[i - 1]
}
