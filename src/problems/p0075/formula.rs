use std::collections::hash_map::Entry;
use rustc_hash::{FxHashMap, FxHashSet};

const LIMIT: i64 = 1_500_000;

pub fn solve() -> i64 {
    let mut solutions = FxHashMap::<i64, FxHashSet<(i64, i64, i64)>>::default();

    for j in 1..LIMIT / 2 {
        for i in j + 1..LIMIT / 2 {
            let a = (i * i) - (j * j);
            let b = 2 * i * j;
            let c = (i * i) + (j * j);
            let l = a + b + c;
            if l > LIMIT {
                break;
            }

            let mut k = 1;
            while k * l <= LIMIT {
                let kl = k * l;
                let (ka, kb) = if a < b { (k * a, k * b) } else { (k * b, k * a) };

                let sides = (ka, kb, k * c);
                if let Entry::Vacant(e) = solutions.entry(kl) {
                    let mut s = FxHashSet::default();
                    s.insert(sides);
                    e.insert(s);
                } else {
                    let s = solutions.get_mut(&kl).unwrap();
                    s.insert(sides);
                }

                k += 1;
            }
        }
    }

    let mut count = 0;
    for (_, v) in solutions {
        if v.len() == 1 {
            count += 1;
        }
    }
    count
}