use std::vec;

use super::common::digit_square_sum;

static LIMIT: i64 = 10_000_000;
static VECTOR_SIZE: usize = ((7 * 81) + 1) as usize;

fn check_chains(n: i64) -> bool {
    let mut m = n;
    while m != 1 && m != 89 {
        m = digit_square_sum(m);
    }

    m == 89
}

pub fn solve() -> i64 {
    let mut result = 0;
    let vsize = VECTOR_SIZE;

    let mut set89 = vec![false; vsize];
    set89[89] = true;

    for i in 1..vsize as i64 {
        if check_chains(i) {
            set89[i as usize] = true;
            result += 1;
        }
    }

    for i in VECTOR_SIZE as i64..LIMIT {
        let s = digit_square_sum(i);
        if set89[s as usize] {
            result += 1;
        }
    }

    result
}
