fn gcd(a: i64, b: i64) -> i64 {
    let mut aa = a;
    let mut bb = b;
    while bb != 0 {
        let t = bb;
        bb = aa % bb;
        aa = t;
    }

    aa
}

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

fn phi(n: i64) -> i64 {
    let mut count = 1;
    for i in 2..n {
        if gcd(n, i) == 1 {
            count += 1;
        }
    }
    count
}

const LIMIT: i64 = 10_000_000;

pub fn solve() -> i64 {
    let mut minimum = f64::MAX;
    let mut result = 0;

    for n in 2..LIMIT {
        let phi_n = phi(n);
        if phi_n < n / 10 {
            continue;
        }

        if is_permutation_to(n, phi_n) {
            let ratio = n as f64 / phi_n as f64;
            if ratio < minimum {
                minimum = ratio;
                result = n;
            }
        }
    }

    result
}
