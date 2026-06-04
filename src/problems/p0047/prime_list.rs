fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

fn build_prime_list(max_prime: i64) -> Vec<i64> {
    let mut list = vec![2, 3, 5, 7, 11, 13, 17, 19];

    let mut n = 23;
    while n <= max_prime {
        let mut is_prime = true;
        for p in &list {
            if p * p > n {
                break;
            }

            if n % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            list.push(n);
        }

        n += 2;
    }

    list
}

fn find_prime_factors(prime_list: &[i64], n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut m = n;

    for p in prime_list {
        if p * p > m {
            break;
        }

        if m % p == 0 {
            factors.push(*p);
            m = remove_factor(m, *p);
        }
    }

    if m > 1 {
        factors.push(m);
    }

    factors
}

pub fn solve() -> i64 {
    let primes = build_prime_list(1000);
    let mut n = 647;
    let mut count = 0;

    loop {
        let factors = find_prime_factors(&primes, n);
        if factors.len() == 4 {
            count += 1;
        } else {
            count = 0;
        }

        if count == 4 {
            break;
        }

        n += 1;
    }

    n - 3
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prime_factors() {
        let primes = build_prime_list(1000);

        assert_eq!(find_prime_factors(&primes, 647), vec![647]);
        assert_eq!(find_prime_factors(&primes, 648), vec![2, 3]);
        assert_eq!(find_prime_factors(&primes, 649), vec![11, 59]);
        assert_eq!(find_prime_factors(&primes, 650), vec![2, 5, 13]);
        assert_eq!(find_prime_factors(&primes, 651), vec![3, 7, 31]);
        assert_eq!(find_prime_factors(&primes, 652), vec![2, 163]);
        assert_eq!(find_prime_factors(&primes, 653), vec![653]);
    }
}
