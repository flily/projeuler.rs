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

fn find_prime_sum_seq(primres: &[i64], n: i64, start: usize) -> i64 {
    for length in start..primres.len() {
        for i in 0..(primres.len() - length) {
            let s: i64 = primres[i..i + length].iter().sum();
            if s == n {
                return length as i64;
            }
        }
    }

    0
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let primes = build_primes_below(LIMIT);
    let mut max_length = 21;
    let mut max_length_prime = 953;

    for prime in primes.iter() {
        let length = find_prime_sum_seq(&primes, *prime, 2);
        if length > max_length {
            max_length = length;
            max_length_prime = *prime;
        }
    }

    max_length_prime
}
