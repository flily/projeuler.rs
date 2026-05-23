fn is_palindrome_10(n: i64) -> bool {
    let size = (n as f64).log10().floor() as u32 + 1;
    for i in 0..(size / 2) {
       let left = (n / 10_i64.pow(i)) % 10;
       let right = (n / 10_i64.pow(size - 1 - i)) % 10;
       if left != right {
           return false;
       }
    }

    true
}

fn is_palindrome_2(n: i64) -> bool {
    let size = (n as f64).log2().floor() as u32 + 1;
    for i in 0..(size / 2) {
       let left = (n / 2_i64.pow(i)) % 2;
       let right = (n / 2_i64.pow(size - 1 - i)) % 2;
       if left != right {
           return false;
       }
    }

    true
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut sum = 0;
    for n in 1..LIMIT {
        if is_palindrome_10(n) && is_palindrome_2(n) {
            sum += n;
        }
    }

    sum
}
