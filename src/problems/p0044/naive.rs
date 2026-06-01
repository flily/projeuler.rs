fn pentagonal(n: i64) -> i64 {
    n * (3 * n - 1) / 2
}

fn is_pentagonal(x: i64) -> bool {
    if x <= 0 {
        false
    } else {
        // n * (3 * n - 1) / 2 = x
        // => n1 = (sqrt(24 * x + 1) + 1) / 6
        // => n2 = - (sqrt(24 * x + 1) - 1) / 6, (n2 < 0, ignore)
        let n = 24 * x + 1;
        let nn = n as f64;
        let sqrt_m = nn.sqrt() as i64;
        sqrt_m * sqrt_m == n && (sqrt_m + 1) % 6 == 0
    }
}

pub fn solve() -> i64 {
    let mut j = 2;
    loop {
        let pj = pentagonal(j);
        for k in 1..j {
            let pk = pentagonal(k);
            if is_pentagonal(pj + pk) && is_pentagonal(pj - pk) {
                return pj - pk;
            }
        }

        j += 1;
    }
}
