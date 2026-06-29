//  n     p            n * q - d * p
// --- - --- = 0  =>  --------------- = 0
//  d     q                d * q
//
// => n * q - d * p = 0
// => n * q = d * p
// => n = d * p / q

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let (p, q) = (3, 7);
    let target = (p as f64) / (q as f64);
    let mut min_diff = 1.0;
    let mut min_n = 0;
    // let mut min_d = 0;

    for d in 3..=LIMIT {
        if d % q == 0 {
            continue;
        }

        let n = (d * p) / q;
        let diff = target - (n as f64) / (d as f64);
        if diff < min_diff {
            min_diff = diff;
            min_n = n;
            // min_d = d;
        }
    }

    min_n
}
