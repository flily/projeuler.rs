fn generate_primes(max_n: i64) -> Vec<i64> {
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19];

    let mut n = 23;
    while n <= max_n {
        let mut is_prime = true;
        for p in &primes {
            if p * p > n {
                break;
            }

            if n % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(n);
        }

        n += 2;
    }

    primes
}

fn count_sum_primes(primes: &[i64], target: i64, max_num: i64) -> i64 {
    if target == 0 {
        1
    } else {
        let mut count = 0;
        for &p in primes {
            if p > target || p > max_num {
                continue;
            }

            count += count_sum_primes(primes, target - p, p);
        }

        count
    }
}

const LIMIT: i64 = 5000;

pub fn solve() -> i64 {
    let mut primes = generate_primes(100);
    primes.reverse();

    let mut n = 11;
    loop {
        let count = count_sum_primes(&primes, n, n - 1);
        if count > LIMIT {
            return n;
        }

        n += 2;
    }
}
