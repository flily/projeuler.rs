use rustc_hash::{FxHashSet, FxHashMap};

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let mut m = 3;
    while m * m <= n {
        if n % m == 0 {
            return false;
        }

        m += 2;
    }

    true
}

fn log10_base(n: i64) -> i64 {
    let mut base_next = 10;
    while n >= base_next {
        base_next *= 10;
    }

    base_next
}

const TOTAL_NUMS: usize = 5;

fn get_main_primes(limit: i64) -> Vec<i64> {
    let mut result = Vec::new();

    // skip 2
    for x in (3..limit).step_by(2) {
        if is_prime(x) {
            result.push(x);
        }
    }

    result
}

fn combination_recursive<F: FnMut(&[i64]) -> Option<()>>(nums: &[i64], state: &mut [bool], index: usize, start: usize, size: usize, result: &mut [i64], callback: &mut F) -> Option<()>{
    if index == size {
        return callback(result);
    }

    for i in start..nums.len() {
        if state[i] {
            continue;
        }

        state[i] = true;
        result[index] = nums[i];
        match combination_recursive(nums, state, index + 1, i + 1, size, result, callback) {
            Some(_) => state[i] = false,
            None => break,
        }
        state[i] = false;
    }

    Some(())
}

fn combination<F: FnMut(&[i64]) -> Option<()>>(nums: &[i64], size: usize, mut callback: F) {
    if size > nums.len() {
        return;
    }

    let mut state = vec![false; nums.len()];
    let mut result = vec![0; size];
    combination_recursive(nums, &mut state, 0, 0, size, &mut result, &mut callback);
}

// correct, use about 43s in release mode.
pub fn solve() -> i64 {
    let main_primes = get_main_primes(10_000);

    let mut prime_pairs = FxHashSet::default();
    let mut prime_pairs_map = FxHashMap::default();
    for i in 0..main_primes.len() {
        for j in (i+1)..main_primes.len() {
            let a = main_primes[i];
            let b = main_primes[j];

            let base_a = log10_base(a);
            let base_b = log10_base(b);

            let ab = a * base_b + b;
            let ba = b * base_a + a;

            if is_prime(ab) && is_prime(ba) {
                prime_pairs.insert((a, b));
                prime_pairs_map.entry(a).or_insert_with(Vec::new).push(b);
                prime_pairs_map.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
    }

    let mut possible_primes = FxHashSet::default();
    for primes in prime_pairs_map.values() {
        if primes.len() >= TOTAL_NUMS - 1 {
            for &p in primes {
                possible_primes.insert(p);
            }
        }
    }


    let mut possible_prime_map = FxHashMap::default();
    for (&k, v) in prime_pairs_map.iter() {
        if !possible_primes.contains(&k) {
            continue;
        }

        let mut prime_list = Vec::new();
        for &p in v {
            if p >= k {
                prime_list.push(p);
            }
        }
        prime_list.push(k);
        prime_list.sort();
        possible_prime_map.insert(k, prime_list);
    }

    let mut min_sum: i64 = -1;
    let mut possible_prime_list = prime_pairs_map.keys().cloned().collect::<Vec<i64>>();
    possible_prime_list.sort();
    for k in possible_prime_list {
        let v = &possible_prime_map[&k];
        combination(v, TOTAL_NUMS, |group| {
            let first = group[0];
            
            let sum: i64 = group.iter().sum::<i64>();
            if min_sum > 0 && sum >= min_sum {
                return None;
            }

            if min_sum > 0 && (first as usize) > (min_sum as usize / group.len()) {
                return None;
            }

            let mut found = true;
            combination(group, 2, |nums| {
                let a = nums[0];
                let b = nums[1];
                if !prime_pairs.contains(&(a, b)) {
                    found = false;
                    return None;
                }

                Some(())
            });

            if found {
                if min_sum < 0 {
                    min_sum = sum;
                } else {
                    min_sum = min_sum.min(sum);
                }
            }

            Some(())
        });
    }

    min_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log10_base() {
        let a = 123;
        let b = 45;
        assert_eq!(10, log10_base(1));
        assert_eq!(1000, log10_base(a));
        assert_eq!(100, log10_base(b));
        assert_eq!(12345, a * log10_base(b) + b);
    }
}
