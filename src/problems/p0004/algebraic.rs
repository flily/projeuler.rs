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
    let mut max_num = 0i64;

    for i in 100..1000 {
        for j in 100..1000 {
            let n = i * j;
            if is_palindrome(n) && n > max_num {
                max_num = n;
            }
        }
    }

    max_num
}

pub fn solve_reverse_vec() -> i64 {
    let mut max_num = 0i64;
    let mut max_j = 999i64;

    for i in (100..1000).rev() {
        for j in (100..i).rev() {
            if max_num != 0 && i < max_j {
                return max_num;
            }

            let n = i * j;
            if n > max_num && is_palindrome(n) {
                max_num = n;
                max_j = j;
            }
        }
    }

    -1
}

pub fn solve_reverse_loop() -> i64 {
    let mut max_num = 0i64;
    let mut max_j = 999i64;

    let (mut i, mut j) = (999i64, 999i64);

    while i >= 100 {
        while j >= 100 {
            if max_num != 0 && i < max_j {
                return max_num;
            }

            let n = i * j;
            if n > max_num && is_palindrome(n) {
                max_num = n;
                max_j = j;
            }

            j -= 1;
        }
        i -= 1;
        j = i;
    }

    -1
}
