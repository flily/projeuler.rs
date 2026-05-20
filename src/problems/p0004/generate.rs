fn make_palindrome(n: i64) -> i64 {
    let mut h = n;
    let mut l = 0;
    let mut m = n;

    while m > 0 {
        let c = m % 10;
        m /= 10;
        h *= 10;
        l = l * 10 + c;
    }

    h + l
}

pub fn solve() -> i64 {
    let mut base = 999;
    while base >= 100 {
        let n = make_palindrome(base);
        let mut i = 999;
        while i >= 100 {
            if n % i == 0 {
                let j = n / i;
                // cargo clippy suggested, better than traditional condition.
                // tested, about 5% (release) ~ 25% (debug) faster than traditional style.
                if (100..=999).contains(&j) {
                    return n;
                }
            }

            i -= 1;
        }

        base -= 1;
    }

    -1
}
