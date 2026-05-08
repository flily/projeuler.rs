fn find_divisors(n: i64) -> i64 {
    // assume n > 1
    let mut count = 2; // 1 and n
    let mut i = 2;

    while i * i < n {
        if n % i == 0 {
            count += 2; // i and n / i
        }

        i += 1;
    }

    if i * i == n {
        count += 1;
    }

    count
}

pub fn solve_3() -> i64 {
    let mut n = 3;
    let mut i = 3;
    let common_factors = 2 * 3 * 5;

    loop {
        if n % common_factors == 0 {
            let d = find_divisors(n);
            if d > 500 {
                return n;
            }
        }

        n += i;
        i += 1;
    }
}

pub fn solve_6() -> i64 {
    let mut n = 3;
    let mut i = 3;
    let common_factors = 2 * 3 * 5 * 7 * 11 * 13;

    loop {
        if n % common_factors == 0 {
            let d = find_divisors(n);
            if d > 500 {
                return n;
            }
        }

        n += i;
        i += 1;
    }
}

pub fn solve_7() -> i64 {
    let mut n = 3;
    let mut i = 3;
    let common_factors = 2 * 3 * 5 * 7 * 11 * 13 * 17;

    loop {
        if n % common_factors == 0 {
            let d = find_divisors(n);
            if d > 500 {
                return n;
            }
        }

        n += i;
        i += 1;
    }
}
