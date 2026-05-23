const POSSIBLE_DIGITS: [i64; 4] = [1, 3, 7, 9];
const POSSIBLE_FIRST_DIGITS: [i64; 4] = [2, 3, 5, 7];
const POSSIBLE_LAST_DIGITS: [i64; 2] = [3, 7];

fn truncate_generator_recursive(size: i64, result: &mut Vec<i64>, index: i64, num: i64) {
    if index == 0 {
        for d in POSSIBLE_FIRST_DIGITS {
            truncate_generator_recursive(size, result, index + 1, d);
        } 
    } else if index == size - 1{
        for d in POSSIBLE_LAST_DIGITS {
            let new_num = num * 10 + d;
            result.push(new_num);
        }
    } else {
        for d in POSSIBLE_DIGITS {
            let new_num = num * 10 + d;
            truncate_generator_recursive(size, result, index + 1, new_num);
        }
    }
}

fn truncate_generator(size: i64) -> Vec<i64> {
    let mut result = Vec::new();

    match size {
        0 => {}
        1 => {
            result.push(2);
            result.push(3);
            result.push(5);
            result.push(7);
        }
        _ => {
            truncate_generator_recursive(size, &mut result, 0, 0);
        }
    }

    result
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt().ceil() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn is_truncatable_prime_left(n: i64) -> bool {
    let mut m = n;
    while m > 0 {
        if !is_prime(m) {
            return false;
        }

        let size = (m as f64).log10().floor() as u32 + 1;
        m %= 10_i64.pow(size - 1);
    }

    true
}

fn is_truncatable_prime_right(n: i64) -> bool {
    let mut m = n;
    while m > 0 {
        if !is_prime(m) {
            return false;
        }

        m /= 10;
    }

    true
}

pub fn solve() -> i64 {
    let mut result = 0;
    for size in 2..7 {
        for n in truncate_generator(size) {
            if is_truncatable_prime_left(n) && is_truncatable_prime_right(n) {
                result += n;
            }
        }
    }

    result
}
