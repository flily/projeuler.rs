use rustc_hash::FxHashMap;

fn count_sum(cache: &mut FxHashMap<(i64, i64), i64>, target: i64, max_num: i64) -> i64 {
    let key = (target, max_num);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    if target == 0 {
        return 1;
    }

    let mut count = 0;
    let mut x = target.min(max_num);
    while x > 0 {
        count += count_sum(cache, target - x, x);
        x -= 1;
    }

    cache.insert(key, count);
    count
}

const TARGET: i64 = 100;

pub fn solve() -> i64 {
    let mut cache = FxHashMap::default();
    count_sum(&mut cache, TARGET, TARGET) - 1
}
