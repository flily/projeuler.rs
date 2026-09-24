fn find_min_multiple(n: i64) -> i64 {
    let mut result = 1;
    while result < n {
        result <<= 1;
    }

    result >> 1
}

pub fn solve() -> i64 {
    let mut factors = (3..21)
        .filter(|x| x % 2 != 0)
        .collect::<Vec<i64>>();

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

    let m = find_min_multiple(20);
    m * factors.iter().product::<i64>()
}
