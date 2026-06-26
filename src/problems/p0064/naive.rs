fn get_period(x: i64) -> i64 {
    let root = (x as f64).sqrt() as i64;
    if root * root == x {
        return 0;
    }

    let mut a = root;
    let (mut n, mut d) = (0, 1);
    let mut period = 0;

    while a != 2 * root {
        n = d * a - n;
        d = (x - n * n) / d;
        a = (root + n) / d;
        period += 1;
    }

    period
}

const LIMIT: i64 = 10000;

pub fn solve() -> i64 {
    let mut count = 0;

    for x in 1..LIMIT {
        let period = get_period(x);
        if period % 2 == 1 {
            count += 1;
        }
    }

    count
}
