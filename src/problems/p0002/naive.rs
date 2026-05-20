static LIMIT: i64 = 4_000_000;

pub fn solve() -> i64 {
    let mut result = 0;

    let (mut a, mut b) = (1, 1);

    while b <= LIMIT {
        if b % 2 == 0 {
            result += b;
        }
        (a, b) = (b, a + b);
    }

    result
}
