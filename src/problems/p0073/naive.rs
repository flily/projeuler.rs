const LIMIT: i64 = 12_000;

fn gcd(a: i64, b: i64) -> i64 {
    let mut bb = b;
    let mut aa = a;
    while bb > 0 {
        (aa, bb) = (bb, aa % bb);
    }

    aa
}

pub fn solve() -> i64 {
    let mut count = 0;

    for d in 4..=LIMIT {
        let lower = d / 3 + 1;
        let upper = (d + 1) / 2;

        for n in lower..upper {
            if gcd(n, d) == 1 {
                count += 1;
            }
        }
    }

    count
}
