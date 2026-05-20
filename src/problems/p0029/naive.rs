use rustc_hash::FxHashSet;
use num_bigint::BigInt;

pub fn solve() -> i64 {
    let mut set = FxHashSet::default();

    for a in 2..=100 {
        for b in 2..=100 {
            let aa = BigInt::from(a);
            let n = aa.pow(b as u32);
            set.insert(n);
        }
    }

    set.len() as i64
}
