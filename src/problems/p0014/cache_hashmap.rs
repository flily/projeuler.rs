use std::collections::HashMap;

fn collatz_seq_size(cache: &mut HashMap<i64, i64>, n: i64) -> i64 {
    let mut m = n;
    let mut result = 1;

    while m > 1 {
        match cache.get(&m) {
            Some(&length) => {
                result += length;
                break;
            }
            None => {
                result += 1;
                m = if m % 2 == 0 { m / 2 } else { 3 * m + 1 };
            }
        }
    }

    cache.insert(n, result);
    result
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut cache = HashMap::with_capacity(LIMIT as usize);
    cache.insert(1, 1);
    cache.insert(2, 2);
    cache.insert(3, 9);
    cache.insert(4, 4);
    let mut max_size = 0;
    let mut result = 0;

    for i in 1..LIMIT {
        let size = collatz_seq_size(&mut cache, i);
        if size > max_size {
            max_size = size;
            result = i;
        }
    }

    result
}
