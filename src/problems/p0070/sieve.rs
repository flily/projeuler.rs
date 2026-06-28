fn get_digit_vector(n: i64, digits: &mut [i32]) {
    let mut num = n;
    while num > 0 {
        let d = (num % 10) as usize;
        digits[d] += 1;
        num /= 10;
    }
}

fn is_permutation_to(a: i64, b: i64) -> bool {
    let mut count_a = [0; 10];
    let mut count_b = [0; 10];

    get_digit_vector(a, &mut count_a);
    get_digit_vector(b, &mut count_b);

    count_a == count_b
}

fn sieve_totient(max_num: i64) -> i64 {
    let mut sieve = vec![0; (max_num + 1) as usize];
    let mut min_nphi = f64::MAX;
    let mut result = 0;
    for i in 2..=(max_num as usize) {
        if sieve[i] == 0 {
            for j in (i..=(max_num as usize)).step_by(i) {
                if sieve[j] == 0 {
                    sieve[j] = j;
                }
                sieve[j] = sieve[j] / i * (i - 1);
            }
        } else {
            let phi = sieve[i];
            if is_permutation_to(i as i64, phi as i64) {
                let np = (i as f64) / (phi as f64);
                if np < min_nphi {
                    min_nphi = np;
                    result = i as i64;
                }
            }
        }
    }

    result
}

const LIMIT: i64 = 10_000_000;

pub fn solve() -> i64 {
    sieve_totient(LIMIT)
}
