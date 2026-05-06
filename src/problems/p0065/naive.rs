use num_bigint::BigInt;

fn digits_sum(n: BigInt) -> i64 {
    let mut sum = BigInt::from(0);
    let mut m = n;
    let zero = BigInt::from(0);

    while m > zero {
        sum += &m % 10;
        m /= 10;
    }

    let (_, u) = sum.to_u64_digits();
    u[0] as i64
}

fn convergents(start: i64, repeat: fn(i64) -> i64, n: i64) -> (BigInt, BigInt) {
    if n <= 1 {
        return (BigInt::from(start), BigInt::from(1));
    }

    let (mut denumerator, mut fraction) = (BigInt::from(1), BigInt::from(repeat(n)));
    let mut i = n - 1;
    while i > 1 {
        let k: i64 = repeat(i);
        (denumerator, fraction) = (fraction.clone(), (fraction * k) + denumerator);
        i -= 1;
    }

    denumerator += &fraction * start;
    (denumerator, fraction)
}

pub fn solve() -> i64 {
    let e_convergent = |x| match (x - 1) % 3 {
        2 => 2 * ((x - 1) / 3 + 1),
        _ => 1,
    };
    let (denumerator, _) = convergents(2, e_convergent, 100);
    digits_sum(denumerator)
}
