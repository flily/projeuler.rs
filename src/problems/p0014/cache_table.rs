fn collatz_seq_size(cache: &mut Vec<i64>, n: i64) -> i64 {
    let mut m = n;
    let mut result = 1i64;

    while m > 1 {
        let item = if m < cache.len() as i64 {
            cache[m as usize]
        } else {
            0
        };

        if 0 == item {
            m = if m % 2 == 0 {
                m / 2
            } else {
                3 * m + 1
            };
            result += 1;

        } else {
            result += item;
            break;
        }
    }

    cache[n as usize] = result;
    return result;
}

const LIMIT: i64 = 1_000_000;

pub fn solve_cache() -> i64 {
    let mut cache = vec![0; (LIMIT + 1) as usize];
    cache[0] = 1;
    cache[1] = 1;
    cache[2] = 2;
    cache[3] = 9;
    cache[4] = 4;
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
    
    use super::super::INFO;
    use super::solve_cache;

    #[test]
    fn test_solve_cache() {
        assert!(INFO.check(solve_cache()));
    }
}
