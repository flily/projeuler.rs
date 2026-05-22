use rustc_hash::FxHashSet;

fn factorial(n: i64) -> i64 {
    let mut result = 1;
    for i in 2..(n + 1) {
        result *= i;
    }

    result
}

fn factorial_sum(n: i64) -> i64 {
    let mut result = 0;
    let mut m = n;

    while m > 0 {
        result += factorial(m % 10);
        m /= 10;
    }

    result
}

fn factorial_sum_chain_size(n: i64) -> i64 {
    let mut count = 0;
    let mut seen = FxHashSet::default();
    let mut m = n;

    while !seen.contains(&m) {
        seen.insert(m);
        m = factorial_sum(m);
        count += 1;
    }

    count
}

pub fn solve() -> i64 {
    let mut count = 0;
    for i in 1..1000000 {
        if factorial_sum_chain_size(i) == 60 {
            count += 1;
        }
    }

    count
}
