fn check_almost_equilateral(a: i128, b: i128, c: i128) -> i64 {
    let s = (a + b + c) / 2;
    let d = s * (s - a) * (s - b) * (s - c);
    let area = (d as f64).sqrt() as i128;
    if area * area == d {
        area as i64
    } else {
        0
    }
}

const LIMIT: i128 = 1_000_000_000;

pub fn solve() -> i64 {
    let mut result = 0;
    for x in (3..=(LIMIT / 3)).step_by(2)  {
        let s1 = check_almost_equilateral(x, x, x - 1);
        if s1 > 0 {
            result += 3 * x - 1;
        }

        let s2 = check_almost_equilateral(x, x, x + 1);
        if s2 > 0 {
            result += 3 * x + 1;
        }
    }

    result as i64
}
