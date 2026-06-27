fn permutation_recursive<F: FnMut(&[i64])>(nums: &[i64], size: usize, state: &mut [bool], index: usize, result: &mut [i64], callback: &mut F){
    if index == size {
        callback(result);
        return;
    }

    for i in 0..nums.len() {
        if state[i] {
            continue;
        }

        state[i] = true;
        result[index] = nums[i];
        permutation_recursive(nums, size, state, index + 1, result, callback);
        state[i] = false;
    }
}

fn permutation<F: FnMut(&[i64])>(nums: &[i64], size: usize, mut callback: F) {
    let mut state = vec![false; nums.len()];
    let mut result = vec![0; nums.len()];
    permutation_recursive(nums, size, &mut state, 0, &mut result, &mut callback)
}

fn gon_flags(size: usize, nums: &[i64]) -> i64 {
    let mut result = String::new();
    for i in 0..size {
        let item = format!("{}{}{}", nums[size + i], nums[i], nums[(i + 1) % size]);
        result.push_str(&item);
    }

    result.parse().unwrap()
}

fn check_gon(size: usize, nums: &[i64]) -> i64 {
    let mut sum = 0;
    for i in 0..size {
        let s = nums[size + i] + nums[i] + nums[(i + 1) % size];
        if sum == 0 {
            sum = s;
        } else if sum != s {
            return 0;
        }
    }

    sum
}


const MIN_17: i64 = 10_000_000_000_000_000;

pub fn solve() -> i64 {
    let size = 5;
    let nums = (1..=10).collect::<Vec<i64>>();

    let mut max_flag = 0;

    permutation(&nums, size * 2, |perm| {
        let mut valid = true;
        for i in 0..(size-1) {
            if perm[size] > perm[size + i + 1] {
                valid = false;
                break;
            }
        }

        if !valid {
            return;
        }

        let sum = check_gon(size, perm);
        if sum > 0 {
            let flag = gon_flags(size, perm);
            if flag < MIN_17 && flag > max_flag {
                max_flag = flag;
            }
        }
    });
    
    max_flag
}
