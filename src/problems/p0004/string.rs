fn is_palindrome(n: i64) -> bool {
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}

pub fn solve() -> i64 {
    let mut max_num = 0;

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
    let mut max_num = 0;
    let mut max_j = 999;

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
    let mut max_num = 0;
    let mut max_j = 999;

    let (mut i, mut j) = (999, 999);

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
