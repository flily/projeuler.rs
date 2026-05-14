use rustc_hash::FxHashSet;

fn sum_of_factors(n: i64) -> i64 {
    let mut sum = 1;
    let mut i = 2;

    while i * i < n {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }

    sum
}

const LIMIT: i64 = 28_123;

pub fn solve() -> i64 {
    let mut abundant_nums = Vec::new();
    let mut abundant_set = FxHashSet::default();

    let mut result = 0;
    for n in 1..LIMIT {
        let sum = sum_of_factors(n);
        if sum > n {
            abundant_nums.push(n);
            abundant_set.insert(n);
        }

        let mut is_sum_of_abundant = false;
        for x in abundant_nums.iter() {
            if *x >= n {
                break;
            }

            let m = n - *x;
            if abundant_set.contains(&m) {
                is_sum_of_abundant = true;
                break;
            }
        }

        if !is_sum_of_abundant {
            result += n;
        }
    }

    result
}
