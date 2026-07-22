use rustc_hash::FxHashMap;

fn partitions(cache: &mut FxHashMap<i64, i64>, modulo: i64, n: i64) -> i64 {
    if let Some(&cached) = cache.get(&n) {
        cached

    } else if n == 0 || n == 1 {
        1

    } else {
        let mut result = 0;

        for x in 1..=n {
            let mut i = (3 * x * x - x) / 2;
            if i > n {
                break;
            }

            let p1 = partitions(cache, modulo, n - i);
            result += if x % 2 == 0 { -p1 } else { p1 };
            result %= modulo;
            i += x;
            if i  > n {
                break;
            }

            let p2 = partitions(cache, modulo, n - i);
            result += if x % 2 == 0 { -p2 } else { p2 };
            result %= modulo;
        }

        cache.insert(n, result);
        result
    }
}

pub fn solve() -> i64 {
    let mut cache = FxHashMap::default();
    let mut n = 5;
    loop {
        let p = partitions(&mut cache, 1_000_000, n);
        if p == 0 {
            return n;
        }
       
        n += 1;
    }
}
