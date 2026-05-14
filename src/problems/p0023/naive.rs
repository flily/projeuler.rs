#[derive(PartialEq)]
enum AbundantType {
    Abundant,
    Deficient,
    Perfect,
}

fn sum_of_factors(n: i64) -> i64 {
    let mut sum = 1;
    let mut i = 2;

    while i * i < n {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }

    sum
}

fn check_type(n: i64) -> AbundantType {
    let sum = sum_of_factors(n);
    if sum > n {
        AbundantType::Abundant
    } else if sum < n {
        AbundantType::Deficient
    } else {
        AbundantType::Perfect
    }
}

const LIMIT: i64 = 28_123;

pub fn solve() -> i64 {
    let mut result = 0;
    let mut n = 1;
    while n < LIMIT {
        let mut found = false;
        for i in 1..n {
            let j = n - i;
            if check_type(i) == AbundantType::Abundant && check_type(j) == AbundantType::Abundant {
                found = true;
                break;
            }
        }
        if !found {
            result += n;
        }
        n += 1;
    }
    result
}
