use rustc_hash::{FxHashMap, FxHashSet};

static FACTORIALS: [i64; 10] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880,
];

fn factorial_sum(n: i64) -> i64 {
    let mut result = 0i64;
    let mut m = n;

    while m > 0 {
        result += FACTORIALS[(m % 10) as usize];
        m /= 10;
    }

    result
}

fn factorial_sum_chain_size(n: i64, cache: &mut FxHashMap<i64, i64>) -> i64 {
    let mut count = 0i64;
    let mut seen = FxHashSet::default();
    let mut m = n;

    while !seen.contains(&m) {
        if cache.contains_key(&m) {
            let result = count + cache[&m];
            cache.insert(n, result);
            return result;
        }

        seen.insert(m);
        m = factorial_sum(m);
        count += 1;
    }

    cache.insert(n, count);
    count
}

pub fn solve() -> i64 {
    let mut count = 0i64;
    let mut cache = FxHashMap::default();
    for i in 1..1000000 {
        if factorial_sum_chain_size(i, &mut cache) == 60 {
            count += 1;
        }
    }

    count
}
