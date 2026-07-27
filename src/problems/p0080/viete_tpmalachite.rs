use malachite::Integer;

fn square_root(n: i64, limit: usize) -> (i64, Vec<i64>) {
    let mut a = 1;
    let mut f = Vec::new();

    let digits = [
        Integer::from(0),
        Integer::from(1),
        Integer::from(2),
        Integer::from(3),
        Integer::from(4),
        Integer::from(5),
        Integer::from(6),
        Integer::from(7),
        Integer::from(8),
        Integer::from(9),
        Integer::from(10),
    ];

    while a * a <= n {
        a += 1;
    }
    a -= 1;

    let remain = n - a * a;
    if remain > 0 {
        let mut r = Integer::from(remain);
        let mut base = Integer::from(a + a);

        let ten = Integer::from(10);
        let hundred = Integer::from(100);

        while f.len() < limit {
            base *= &ten;
            r *= &hundred;

            let mut d = 1;
            while (&base + &digits[d]) * &digits[d] < r {
                d += 1;
            }

            d -= 1;
            f.push(d as i64);
            r -= (&base + &digits[d]) * &digits[d];
            base += &digits[d] + &digits[d];
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
