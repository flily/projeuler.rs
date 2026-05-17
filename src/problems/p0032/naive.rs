use rustc_hash::FxHashSet;

fn mark_digit(digits: &mut [bool; 9], n: i64) -> bool {
    let mut n = n;
    while n > 0 {
        let d = (n % 10) as usize;
        if d == 0 || digits[d - 1] {
            return false;
        }
        digits[d - 1] = true;
        n /= 10;
    }

    true
}

fn check_pandigital(prod: i64, a: i64, b: i64) -> bool {
    if a == 0 || b == 0 {
        return false;
    }
    let mut digits = [false; 9];
    if !mark_digit(&mut digits, a) {
        return false;
    }

    if !mark_digit(&mut digits, b) {
        return false;
    }

    if !mark_digit(&mut digits, prod) {
        return false;
    }

    digits.iter().all(|d| *d)
}

pub fn solve() -> i64 {
    let mut set = FxHashSet::default();

    for a in 1..1000 {
        for b in 1..10000 {
            let prod = a * b;
            if check_pandigital(prod, a, b) {
                set.insert(prod);
            }
        }
    }

    set.iter().sum()
}
