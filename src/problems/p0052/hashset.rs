use rustc_hash::FxHashSet;

fn get_digits(n: i64) -> FxHashSet<i64> {
    let mut digits = FxHashSet::default();
    let mut m = n;
    while m > 0 {
        digits.insert(m % 10);
        m /= 10;
    }

    digits
}

pub fn solve() -> i64 {
    let mut n = 2;
    loop {
        let n_digits = get_digits(n);
        let mut found = true;
        for i in 2..=6 {
            let m = n * i;
            let m_digits = get_digits(m);
            if n_digits.len() != m_digits.len() {
                found = false;
                break;
            }

            if n_digits != m_digits {
                found = false;
                break;
            }
        }

        if found {
            return n;
        }
        n += 1;
    }
}
