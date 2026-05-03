use super::common;

fn check_chains(n: i64) -> bool {
    let mut m = n;
    while m != 1 && m != 89 {
        m = common::digit_square_sum(m);
    }

    m == 89
}

static LIMIT: i64 = 10_000_000;

pub fn solve() -> i64 {
    let mut result = 0i64;

    for i in 1..LIMIT {
        if check_chains(i) {
            result += 1;
        }
    }

    result
}
