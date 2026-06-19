use std::str::FromStr;

use super::nums::NUMS;

use malachite::Integer;

fn nums_sum() -> Integer {
    let mut sum = Integer::from(0);
    for n in NUMS {
        let num = Integer::from_str(n).unwrap();
        sum += num;
    }

    sum
}

pub fn solve() -> i64 {
    let sum = nums_sum();
    let result_string = sum.to_string();
    result_string[..10].parse().unwrap()
}
