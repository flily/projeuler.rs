use rustc_hash::FxHashSet;

fn permutation_recursive<F: FnMut(&[i64])>(nums: &[i64], state: &mut [bool], index: usize, result: &mut [i64], callback: &mut F){
    if index == nums.len() {
        callback(result);
        return;
    }

    for i in 0..nums.len() {
        if state[i] {
            continue;
        }

        state[i] = true;
        result[index] = nums[i];
        permutation_recursive(nums, state, index + 1, result, callback);
        state[i] = false;
    }
}

fn permutation<F: FnMut(&[i64])>(nums: &[i64], mut callback: F) {
    let mut state = vec![false; nums.len()];
    let mut result = vec![0; nums.len()];
    permutation_recursive(nums, &mut state, 0, &mut result, &mut callback)
}

fn combination_recursive<F: FnMut(&[i64])>(nums: &[i64], state: &mut [bool], index: usize, start: usize, size: usize, result: &mut [i64], callback: &mut F) {
    if index == size {
        callback(result);
        return;
    }

    for i in start..nums.len() {
        if state[i] {
            continue;
        }

        state[i] = true;
        result[index] = nums[i];
        combination_recursive(nums, state, index + 1, i + 1, size, result, callback);
        state[i] = false;
    }
}

fn combination<F: FnMut(&[i64])>(nums: &[i64], size: usize, mut callback: F) {
    if size > nums.len() {
        return;
    }

    let mut state = vec![false; nums.len()];
    let mut result = vec![0; size];
    combination_recursive(nums, &mut state, 0, 0, size, &mut result, &mut callback)
}

fn is_prime(n: i64) -> bool {
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

fn get_digits(n: i64) -> Vec<i64> {
    let mut digits = Vec::new();
    let mut m = n;
    while m > 0 {
        digits.push(m % 10);
        m /= 10;
    }

    digits
}

const KNOWN_ANSWER: i64 = 1487;

pub fn solve() -> i64 {
    let mut n = 1001;
    while n < 10000 {
        if !is_prime(n) {
            n += 2;
            continue;
        }

        let mut pp = FxHashSet::default();
        let digits = get_digits(n);
        permutation(&digits, |ds| {
            if ds[0] == 0 || ds[3] % 2 == 0 {
                return;
            }

            let m = ds[0] * 1000 + ds[1] * 100 + ds[2] * 10 + ds[3];
            if is_prime(m) {
                pp.insert(m);
            }
        });

        if pp.len() < 3 {
            n += 2;
            continue;
        }

        let mut pl = pp.iter().cloned().collect::<Vec<i64>>();
        pl.sort();

        let mut result = None;
        combination(&pl, 3, |nums| {
            if nums[1] - nums[0] == nums[2] - nums[1] && nums[0] != KNOWN_ANSWER {
                result = Some(nums[0] * 100_000_000 + nums[1] * 10_000 + nums[2]);
                
            }
        });

        if let Some(r) = result {
            return r;
        }
        n += 2;
    }

    0
}
