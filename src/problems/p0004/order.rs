fn is_palindrome(n: i64) -> bool {
    if n / 100_000 != n % 10 {
        return false;
    }

    if (n / 10_000) % 10 != (n % 100) / 10 {
        return false;
    }

    if (n / 1000) % 10 != (n % 1000) / 100 {
        return false;
    }

    true
}

pub fn solve() -> i64 {
    let mut i = 0i64;
    let mut j = 0i64;

    while i < 999 && j < 999 {
        let mut x = 999 - i;
        let mut y = 999 - j;
        while x < 1000 {
            let n = x * y;
            if is_palindrome(n) {
                return n;
            }

            x += 1;
            y -= 1;
        }

        if i == j {
            j += 1;
        } else {
            i += 1;
        }
    }

    -1
}