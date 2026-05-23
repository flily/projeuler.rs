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

fn palindrome_gen_n(n: i64) -> Vec<i64> {
    let mut result = Vec::new();

    match n {
        0 => {
            result.push(1)
        }
        1 => {
            for x in 2..10 {
                result.push(x);
            }
        }
        _ => {
            let half_n = n as u32 / 2;
            let size = (n as f64 / 2.0).ceil() as u32;
            for x in 10_i64.pow(size - 1)..10_i64.pow(size) {
                let mut y = 0;
                for i in 0..half_n {
                    let di = (x / 10_i64.pow(size - i - 1)) % 10;
                    y += di * 10_i64.pow(i);
                }

                let m = x * 10_i64.pow(half_n) + y;
                result.push(m);
            }
        }
    }

    result
}

pub fn solve() -> i64 {
    let mut result = 0;
    for i in 0..7 {
        for n in palindrome_gen_n(i) {
            if is_palindrome_2(n) {
                result += n;
            }
        }
    }

    result
}
