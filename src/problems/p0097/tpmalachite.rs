use malachite::{Integer, base::num::arithmetic::traits::Pow};

pub fn solve() -> i64 {
    let m = Integer::from(10).pow(10);
    let mut result = Integer::from(2).pow(7830457) * Integer::from(28433) + Integer::from(1);
    result %= m;
    result.to_string().parse::<i64>().unwrap()
}
