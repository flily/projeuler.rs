static LIMIT: usize = 10001;

pub fn solve() -> i64 {
    let mut primes = vec![3, 5, 7, 11, 13, 17, 19];

    for n in (23..).step_by(2) {
        let mut is_prime = true;
        for &p in &primes {
            if n % p == 0 {
                is_prime = false;
                break;
            }

            if p * p > n {
                break;
            }
        }

        if is_prime {
            primes.push(n);
        }

        if primes.len() >= LIMIT - 1 {
            // 2 is not counted
            break;
        }
    }

    *primes.last().unwrap()
}
