use num_bigint::BigInt;

fn square_root(n: i64, limit: usize) -> (i64, Vec<i64>) {
    let mut a = 1;
    let mut f = Vec::new();

    while a * a <= n {
        a += 1;
    }
    a -= 1;

    let remain = n - a * a;
    if remain > 0 {
        let mut r = BigInt::from(remain);
        let mut base = BigInt::from(a + a);
        while f.len() < limit {
            base *= 10;
            r *= 100;

            let mut d = 1;
            while (&base + d) * d < r {
                d += 1;
            }

            d -= 1;
            f.push(d);
            r -= (&base + d) * d;
            base += 2 * d;
        }
    }

    (a, f)
}

pub fn solve() -> i64 {
    let mut result = 0;

    for n in 2..=100 {
        let (i, f) = square_root(n, 99);
        if !f.is_empty() {
            let s = f.iter().sum::<i64>();
            result += i + s;
        }
    }

    result
}
