use malachite::Integer;
use malachite::base::num::arithmetic::traits::Pow;

pub fn solve() -> i64 {
    let mut n = Integer::from(2);
    n = n.pow(1000);

    n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .sum()
}