fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn permutation_recursive(nums: &[i64], state: &mut [bool], index: usize, result: &mut [i64], callback: fn(&[i64]) -> Option<i64>) -> Option<i64> {
    if index == nums.len() {
        return callback(result);
    }

    for i in 0..nums.len() {
        if state[i] {
            continue;
        }

        state[i] = true;
        result[index] = nums[i];
        let r = permutation_recursive(nums, state, index + 1, result, callback);
        if r.is_some() {
            return r;
        }
        state[i] = false;
    }

    None
}

fn permutation(nums: &[i64], callback: fn(&[i64]) -> Option<i64>) -> Option<i64> {
    let mut state = vec![false; nums.len()];
    let mut result = vec![0; nums.len()];
    permutation_recursive(nums, &mut state, 0, &mut result, callback)
}

pub fn solve() -> i64 {
    for size in (1..=9).rev() {
        let digits = (1..=size).map(|x| x as i64).rev().collect::<Vec<i64>>();

        let r = permutation(&digits, |nums| {
            let mut n = 0;
            for num in nums {
                n = n * 10 + num;
            }

            if is_prime(n) {
                Some(n)
            } else {
                None
            }
        });

        if let Some(n) = r {
            return n;
        }
    }

    0
}
