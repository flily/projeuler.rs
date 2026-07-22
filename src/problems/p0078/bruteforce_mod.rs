use rustc_hash::FxHashMap;

fn count_sum(cache: &mut FxHashMap<(i64, i64), i64>, m: i64, target: i64, max_num: i64) -> i64 {
    let key = (target, max_num);
    if let Some(&cached) = cache.get(&key) {
        cached

    } else if target == 0 {
        1

    } else {
        let mut count = 0;
        let mut x = target.min(max_num);
        while x > 0 {
            let new_target = target - x;
            let c = count_sum(cache, m, new_target, new_target.min(x));
            count = (count + c) % m;
            x -= 1;
        }

        cache.insert(key, count);
        count
    }
}

pub fn solve() -> i64 {
    let mut cache = FxHashMap::default();
    let mut n = 19;
    let m = 1_000_000;
    loop {
        let p = count_sum(&mut cache, m, n, n);
        if p == 0 {
            return n;
        }

        n += 1;
    }
}
