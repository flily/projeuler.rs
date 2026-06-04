use num_bigint::BigInt;

const LIMIT: u32 = 1000;
const MODULO: i64 = 10_000_000_000;

pub fn solve() -> i64 {
    let mut sum = BigInt::from(0);

    for i in 1..=LIMIT {
        let p = BigInt::from(i).pow(i);
        sum += p;
    }

    let result = sum % MODULO;
    result.to_string().parse::<i64>().unwrap()
}
