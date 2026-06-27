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

fn relative_primes(n: i64) -> i64 {
    let mut count = 0;
    for i in 1..n {
        if gcd(i, n) == 1 {
            count += 1;
        }
    }
    count
}

fn nphi(n: i64) -> f64 {
    let rp = relative_primes(n);
    (n as f64) / (rp as f64)
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut max_phi = 0.0;
    let mut result = 0;

    for n in 2..=LIMIT {
        let np = nphi(n);
        if np > max_phi {
            max_phi = np;
            result = n;
        }
    }

    result
}
