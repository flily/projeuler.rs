pub fn solve() -> i64 {
    let mut factors = (2..21).collect::<Vec<i64>>();

    for i in 0..factors.len() {
        let n = factors[i];
        if n == 1 {
            continue;
        }

        for item in factors.iter_mut().skip(i + 1) {
            if *item % n == 0 {
                *item /= n;
            }
        }
    }

    factors.iter().product()
}
