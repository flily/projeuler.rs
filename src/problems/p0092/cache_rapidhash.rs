use rapidhash::RapidHashSet;

use super::common::digit_square_sum;

fn check_chains(n: i64, set1: &mut RapidHashSet<i64>, set89: &mut RapidHashSet<i64>) -> bool {
    let mut m = n;
    let mut ok1 = set1.contains(&m);
    let mut ok89 = set89.contains(&m);

    while !ok1 && !ok89 {
        m = digit_square_sum(m);
        ok1 = set1.contains(&m);
        ok89 = set89.contains(&m);
    }

    if ok89 {
        set89.insert(n);
    } else {
        set1.insert(n);
    }

    ok89
}

static LIMIT: i64 = 10_000_000;

pub fn solve() -> i64 {
    let mut result = 0i64;

    let mut set1 = RapidHashSet::with_capacity(LIMIT as usize);
    let mut set89 = RapidHashSet::with_capacity(LIMIT as usize);
    set1.insert(1);
    set89.insert(89);

    for i in 1..LIMIT {
        if check_chains(i, &mut set1, &mut set89) {
            result += 1;
        }
    }

    result
}
