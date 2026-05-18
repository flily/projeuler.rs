fn cancel_fraction(m: i64, n: i64) -> Option<(i64, i64)> {
    let m1 = m / 10;
    let m2 = m % 10;
    let n1 = n / 10;
    let n2 = n % 10;

    if m1 == n1 {
        Some((m2, n2))
    } else if m1 == n2 {
        Some((m2, n1))
    } else if m2 == n1 {
        Some((m1, n2))
    } else if m2 == n2 {
        Some((m1, n1))
    } else {
        None
    }
}


pub fn solve() -> i64 {
    let mut p = 1;
    let mut q = 1;

    for m in 10..100 {
        for n in (m + 1)..100 {
            if m % 10 == 0 && n % 10 == 0 {
                continue;
            }

            if let Some((m1, n1)) = cancel_fraction(m, n) && m * n1 == n * m1 {
                p *= m1;
                q *= n1;
            }
        }
    }

    q / p
}
