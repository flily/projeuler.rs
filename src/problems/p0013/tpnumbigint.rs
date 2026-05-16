use super::nums::NUMS;

use num_bigint::BigInt;

fn nums_sum() -> BigInt {
     let mut sum = BigInt::from(0);
    for n in NUMS {
        let num = BigInt::parse_bytes(n.as_bytes(), 10).unwrap();
        sum += num;
    }

    sum
}

pub fn solve() -> i64 {
    let sum = nums_sum();
    let result_string = sum.to_string();
    result_string[..10].parse::<i64>().unwrap()
}
