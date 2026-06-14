fn get_digits(n: i64) -> [i64; 10] {
    let mut digits = [0; 10];
    let mut m = n;
    while m > 0 {
        digits[(m % 10) as usize] += 1;
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
