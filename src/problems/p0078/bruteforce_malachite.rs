use rustc_hash::FxHashMap;
use malachite::Integer;

fn count_sum(cache: &mut FxHashMap<(i64, i64), Integer>, target: i64, max_num: i64) -> Integer {
    let key = (target, max_num);
    if let Some(cached) = cache.get(&key) {
        cached.clone()

    } else if target == 0 {
        Integer::from(1)

    } else {
        let mut count = Integer::from(0);
        let mut x = target.min(max_num);
        while x > 0 {
            let new_target = target - x;
            count += count_sum(cache, new_target, new_target.min(x));
            x -= 1;
        }

        cache.insert(key, count.clone());
        count
    }
}

pub fn solve() -> i64 {
    let mut cache = FxHashMap::default();
    let mut n = 19;
    let million = Integer::from(1_000_000);
    let zero = Integer::from(0);
    loop {
        let p = count_sum(&mut cache, n, n);
        if p % &million == zero {
            return n;
        }

        n += 1;
    }
}
