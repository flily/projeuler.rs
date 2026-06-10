const LIMIT: i64 = 4_000_000;

pub fn solve() -> i64 {
    let mut result = 0;
    let (mut a, mut b) = (1, 2);

    while b <= LIMIT {
        result += b;
        (a, b) = (b, a + b);
        (a, b) = (b, a + b);
        (a, b) = (b, a + b);
    }

    result
}
