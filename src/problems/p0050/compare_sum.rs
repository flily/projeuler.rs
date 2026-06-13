fn is_prime(primes: &[i64], n: i64) -> bool {
    for i in primes.iter().skip(1) {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            return false;
        }
    }

    true
}

fn build_primes_below(n: i64) -> Vec<i64> {
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    for i in (21..n).step_by(2) {
        if is_prime(&primes, i) {
            primes.push(i);
        }
    }
    primes
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let primes = build_primes_below(LIMIT);
    let mut max_length = 21;
    let mut max_length_prime = 953;

    for end in (21..primes.len()).rev() {
        let mut s = 0;
        let mut start = end;
        while start > 0 {
            let p = primes[start - 1];
            if s + p >= LIMIT {
                break;
            }

            s += p;
            start -= 1;
        }

        if end - start < max_length || end < max_length {
            continue;
        }

        while start < end {
            if primes.binary_search(&s).is_ok() {
                let length = end - start;
                if length > max_length {
                    max_length = length;
                    max_length_prime = s;
                }

                break;
            }

            s -= primes[start];
            start += 1;
        }
    }

    max_length_prime
}