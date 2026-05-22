// Pell's equation: x^2 - D*y^2 = 1

fn find_solution(d: i128) -> (i128, i128) {
    let mut m = 0;
    let mut dd = 1;
    let a0 = (d as f64).sqrt() as i128;
    let mut a1 = a0;

    let (mut x1, mut x0) = (1, a1);
    let (mut y1, mut y0) = (0, 1);

    while x0 * x0 - d * y0 * y0 != 1 {
        m = dd * a1 - m;
        dd = (d - m * m) / dd;
        a1 = (a0 + m) / dd;

        (x1, x0) = (x0, a1 * x0 + x1);
        (y1, y0) = (y0, a1 * y0 + y1);
    }

    (x0, y0)
}

pub fn solve() -> i64 {
    let squares = (1..32).map(|x| x * x).collect::<Vec<_>>();
    let mut max_x = 0;
    let mut index = 0;

    for d in 1..=1000 {
        if squares.binary_search(&d).is_ok() {
            continue;
        }

        let (x, _) = find_solution(d as i128);
        if x > max_x {
            max_x = x;
            index = d;
        }
    }

    index
}
