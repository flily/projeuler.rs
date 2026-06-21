use num_bigint::BigInt;

fn digit_sum(n: &BigInt) -> i64 {
    let s = n.to_string();
    let mut sum = 0;

    for c in s.chars() {
        sum += (c as i64) - 0x30;
    }

    sum
}

pub fn solve() -> i64 {
    let mut max_sum = 0;
    for a in 2..100 {
        let aa = BigInt::from(a);

        for b in 2..100 {
           let n = aa.pow(b);
           let s = digit_sum(&n);
           max_sum = max_sum.max(s); 
        }
    }

    max_sum
}
