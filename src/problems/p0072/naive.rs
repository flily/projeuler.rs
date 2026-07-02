fn gcd(a: i64, b: i64) -> i64 {
    let mut aa = a;
    let mut bb = b;
    while bb != 0 {
        (aa, bb) = (bb, aa % bb);
    }

    aa
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut count = 0;

    for d in 2..=LIMIT {
        for n in 1..d {
            if gcd(n, d) == 1 {
                count += 1;
            }
        }
    }

    count
}
