use rustc_hash::FxHashMap;
use malachite::Integer;

fn partitions(cache: &mut FxHashMap<i64, Integer>, n: i64) -> Integer {
    if let Some(cached) = cache.get(&n) {
        cached.clone()

    } else if n == 0 || n == 1 {
        Integer::from(1)

    } else {
        let mut result = Integer::from(0);

        for x in 1..=n {
            let mut i = (3 * x * x - x) / 2;
            if i > n {
                break;
            }

            let p1 = partitions(cache, n - i);
            result += if x % 2 == 0 { -p1 } else { p1 };
            i += x;
            if i  > n {
                break;
            }

            let p2 = partitions(cache, n - i);
            result += if x % 2 == 0 { -p2 } else { p2 };
        }

        cache.insert(n, result.clone());
        result
    }
}

pub fn solve() -> i64 {
    let mut cache = FxHashMap::default();
    let mut n = 5;
    let million = Integer::from(1_000_000);
    let zero = Integer::from(0);
    loop {
        let p = partitions(&mut cache, n);
        if p % &million == zero {
            return n;
        }
        n += 1;
    }
}
