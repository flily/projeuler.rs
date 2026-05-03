use std::collections::HashMap;

fn collatz_seq_size(cache: &mut HashMap<i64, i64>, n: i64) -> i64 {
    let mut m = n;
    let mut result = 1i64;

    while m > 1 {
        match cache.get(&m) {
            Some(&length) => {
                result += length;
                break
            }
            None => {
                result += 1;
                m = if m % 2 == 0 {
                    m / 2
                } else {
                    3 * m + 1
                };
            }
        }
    }

    cache.insert(n, result);
    return result;
}

const LIMIT: i64 = 1_000_000;

pub fn solve_cache() -> i64 {
    let mut cache = HashMap::with_capacity(2 * LIMIT as usize);
    cache.insert(1, 1);
    cache.insert(2, 2);
    cache.insert(3, 9);
    cache.insert(4, 4);
    let mut max_size = 0i64;
    let mut result = 0i64;

    for i in 1..LIMIT {
        let size = collatz_seq_size(&mut cache, i);
        if size > max_size {
            max_size = size;
            result = i;
        }
    }

    return result;
}


#[cfg(test)]
mod tests {
    use crate::common::Checkable;
    use crate::problems::p0014::cache::solve_cache;

    use super::super::INFO;

    #[test]
    fn test_solve_cache() {
        assert!(INFO.check(solve_cache()));
    }
}
