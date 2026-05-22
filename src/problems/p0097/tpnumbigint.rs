use num_bigint::BigInt;

pub fn solve() -> i64 {
    let m = BigInt::from(10).pow(10);
    let mut result = BigInt::from(2).pow(7830457) * BigInt::from(28433) + BigInt::from(1);
    result %= m;
    result.to_u64_digits().1[0] as i64
}
