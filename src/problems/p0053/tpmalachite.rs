use malachite::Integer;

fn factorial(n: i64) -> Integer {
    let mut result = Integer::from(1);
    for i in 2..=n {
        result *= Integer::from(i);
    }
    result
}

fn combinations(n: i64, r: i64) -> Integer {
    factorial(n) / (factorial(r) * factorial(n - r))
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut result = 0;
    for n in 1..=100 {
        for r in 1..=n {
            if combinations(n, r) > LIMIT {
                result += 1;
            }
        }
    }

    result
}

fn factorial_table(n: usize) -> Vec<Integer> {
    let mut table = Vec::with_capacity(n + 1);
    let mut result = Integer::from(1);
    table.push(result.clone()); // factorial(0) = 1
    table.push(result.clone()); // factorial(1) = 1

    for i in 2..=n {
        result *= Integer::from(i as i64);
        table.push(result.clone());
    }

    table
}

fn combinations_table(factorials: &[Integer], n: usize, r: usize) -> Integer {
    &factorials[n] / (&factorials[r] * &factorials[n - r])
}

pub fn solve_precalculated() -> i64 {
    let factors = factorial_table(100);
    let mut result = 0;
    let limit = Integer::from(LIMIT);

    for n in 1..=100 {
        for r in 1..=n {
            if combinations_table(&factors, n as usize, r as usize) > limit {
                result += 1;
            }
        }
    }

    result
}
