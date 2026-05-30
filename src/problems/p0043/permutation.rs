const PRIMES: [i64; 7] = [2, 3, 5, 7, 11, 13, 17];

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

pub fn solve() -> i64 {
    let mut result = 0;
    let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    permutation(&digits, |nums| {
        if nums[0] == 0 {
            return;
        }

        let mut found = true;
        for i in 0..7 {
            let n = nums[i + 1] * 100 + nums[i + 2] * 10 + nums[i + 3];
            if n % PRIMES[i] != 0 {
                found = false;
                break;
            }
        }

        if found {
            let mut m = 0;
            nums.iter().take(10).for_each(|&x| m = m * 10 + x);

            result += m;
        }
    });

    result
}
