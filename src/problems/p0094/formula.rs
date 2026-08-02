fn seq_add(limit: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let (mut a, mut b) = (1, 5);
    while b < limit {
        result.push(b);
        (a, b) = (b, 14 * b - a - 4);
    }

    result
}

fn seq_sub(limit: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let (mut a, mut b) = (1, 17);
    while b < limit {
        result.push(b);
        (a, b) = (b, 14 * b - a + 4);
    }

    result
}

const LIMIT: i64 = 1_000_000_000;

pub fn solve() -> i64 {
    let mut result = 0;

    for x in seq_add(LIMIT / 3) {
        result += 3 * x + 1;
    }

    for x in seq_sub(LIMIT / 3) {
        result += 3 * x - 1;
    }

    result
}
