use malachite::Integer;
use malachite::base::num::arithmetic::traits::Pow;

const LIMIT: u64 = 1000;
const MODULO: i64 = 10_000_000_000;

pub fn solve() -> i64 {
    let mut sum = Integer::from(0);

    for i in 1..=LIMIT {
        let p = Integer::from(i).pow(i);
        sum += p;
    }

    let result = sum % Integer::from(MODULO);
    result.to_string().parse::<i64>().unwrap()
}
