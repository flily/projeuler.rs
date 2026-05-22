use std::ops::Mul;

use malachite::{
    Integer,
    base::num::arithmetic::traits::FloorSqrt,
};

fn to_i64(x: &Integer) -> i64 {
    let sn = x.to_string();
    sn.parse::<i64>().unwrap()
}

// When D=109, x=158_070_671_986_249, y=15_140_424_455_100
// It is impossible to find by brute-force.
fn find_minimal_x_bigint(d: i64) -> (i64, i64) {
    let mut y = 1;
    let one = Integer::from(1);
    loop {
        let by = Integer::from(y);
        let x2 = &by * &by * Integer::from(d) + &one;
        let x = (&x2).floor_sqrt();
        if (&x * &x) == x2 {
            let rx = to_i64(&x);
            return (rx, y);
        }

        y += 1;
    }
}

pub fn solve() -> i64 {
    let squares = (1..32).map(|x| x * x).collect::<Vec<_>>();
    let mut max_x = 0;
    let mut result = 0;

    for d in 2..1001 {
        if squares.binary_search(&d).is_ok() {
            continue;
        }

        let (x, y) = find_minimal_x_bigint(d);
        let r1 = x * x;
        let r2 = y.mul(d).mul(y);
        let r = r1 - r2;
        if r != 1 {
            continue;
        }
        if x > max_x {
            max_x = x;
            result = d;
        }
    }

    result
}
