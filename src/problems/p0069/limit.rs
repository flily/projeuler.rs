fn gcd(a: i64, b: i64) -> i64 {
    let mut aa = a;
    let mut bb = b;
    while bb != 0 {
        let t = bb;
        bb = aa % bb;
        aa = t;
    }

    aa
}

fn nphi_limit(n: i64, max_nphi: f64) -> f64 {
    let mut count = 0;
    for i in (1..n).step_by(2) {
        if gcd(i, n) == 1 {
            count += 1;
        }

        if i > 300 && (n as f64) / (count as f64) < max_nphi {
            return 0.0;
        }
    }

    for i in (2..n).step_by(2) {
        if gcd(i, n) == 1 {
            count += 1;
        }
    }

    (n as f64) / (count as f64)
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut max_phi = 0.0;
    let mut result = 0;

    for n in 2..=LIMIT {
        let np = nphi_limit(n, max_phi);
        if np > max_phi {
            max_phi = np;
            result = n;
        }
    }

    result
}
