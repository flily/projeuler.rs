fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        return false;
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

fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

fn find_prime_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut m = n;
    if m % 2 == 0 {
        factors.push(2);
        m = remove_factor(m, 2);
    }

    let mut i = 3;
    while i * i <= n {
        if m % i == 0 && is_prime(i) {
            factors.push(i);
            m = remove_factor(m, i);
        }
        i += 2;
    }

    if m > 1 && is_prime(m) {
        factors.push(m);
    }

    factors
}

pub fn solve() -> i64 {
    let mut n = 647;
    let mut count = 0;

    loop {
        let factors = find_prime_factors(n);
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
        assert_eq!(find_prime_factors(647), vec![647]);
        assert_eq!(find_prime_factors(648), vec![2, 3]);
        assert_eq!(find_prime_factors(649), vec![11, 59]);
        assert_eq!(find_prime_factors(650), vec![2, 5, 13]);
        assert_eq!(find_prime_factors(651), vec![3, 7, 31]);
        assert_eq!(find_prime_factors(652), vec![2, 163]);
        assert_eq!(find_prime_factors(653), vec![653]);
    }
}
