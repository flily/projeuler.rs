fn power_mod(base: i64, exp: i64, module: i64) -> i64 {
    let mut result = 1;
    let mut e = exp;
    while e > 0 {
        result = (result * base) % module;
        e -= 1;
    }

    result
}

const LIMIT: i64 = 1000;
const MODULO: i64 = 10_000_000_000;

pub fn solve() -> i64 {
    let mut sum = 0;
    for i in 1..=LIMIT {
        let p = power_mod(i, i, MODULO);
        sum = (sum + p) % MODULO;
    }
    sum
}
