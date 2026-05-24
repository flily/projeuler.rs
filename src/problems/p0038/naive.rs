fn is_pandigital_9(n: i64) -> bool {
    if !(123456789..=987654321).contains(&n) {
        return false;
    }

    let mut digits = [false; 10];
    let mut m = n;
    while m > 0 {
        let d = (m % 10) as usize;
        if d == 0 || digits[d] {
            return false;
        }
        digits[d] = true;
        m /= 10;
    }

    true
}

fn num_digits(n: i64) -> i64 {
    (n as f64).log10().floor() as i64 + 1
}

fn concatenate_product(nums: &[i64]) -> i64 {
    let mut size = 0;
    for n in nums {
        size += num_digits(*n);
    }
    
    let mut result = 0;
    let mut left = size;
    for n in nums {
        left -= num_digits(*n);
        result += n * 10_i64.pow(left as u32);
    }
    result
}

pub fn solve() -> i64 {
    let mut result = 0;
    for n in 10..10_000 {
        let mut digits = 0;
        let mut nums = Vec::new();

        for i in 1..10 {
            let d = n * i;
            digits += num_digits(d);
            nums.push(d);
            if digits >= 9 {
                break;
            }
        }

        if digits == 9 {
            let candidate = concatenate_product(&nums);
            if is_pandigital_9(candidate) {
                result = result.max(candidate);
            }
        }
    }

    result
}
