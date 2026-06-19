fn make_palindrome(n: i64) -> i64 {
    let mut result = 0;
    let mut m = n;

    while m > 0 {
        result = result * 10 + m % 10;
        m /= 10;
    }

    result
}

fn is_palindrome(n: i64) -> bool {
    n == make_palindrome(n)
}

fn is_lychrel(n: i64) -> bool {
    let mut m = n;
    for _ in 0..50 {
        m = m + make_palindrome(m);
        if is_palindrome(m) {
            return false;
        }
    }

    true
}

pub fn solve() -> i64 {
    let mut count = 0;
    for i in 1..10_000 {
        if is_lychrel(i) {
            count += 1;
        }
    }

    count
}
