use std::vec;

use super::common::digit_square_sum;

fn check_chains(n: i64, set1: &mut [u8], set89: &mut [u8]) -> bool {
    let mut m = n;
    let mut ok1 = set1[m as usize] == 1;
    let mut ok89 = set89[m as usize] == 1;

    while !ok1 && !ok89 {
        m = digit_square_sum(m);
        ok1 = set1[m as usize] == 1;
        ok89 = set89[m as usize] == 1;
    }

    if ok89 {
        set89[n as usize] = 1;
    } else {
        set1[n as usize] = 1;
    }

    ok89
}

static LIMIT: i64 = 10_000_000;

pub fn solve() -> i64 {
    let mut result = 0;

    let mut set1 = vec![0; (LIMIT + 1) as usize];
    let mut set89 = vec![0; (LIMIT + 1) as usize];
    set1[1] = 1;
    set89[89] = 1;

    for i in 1..LIMIT {
        if check_chains(i, &mut set1, &mut set89) {
            result += 1;
        }
    }

    result
}
