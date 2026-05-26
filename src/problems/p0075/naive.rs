fn find(l: i64) -> i64 {
    let mut count = 0;

    for a in 1..(l / 2) {
        for b in 1..a {
            let c = l - a - b;
            if c < b {
                break;
            }

            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }

    count
}

static LIMIT: i64 = 1_500_000;

pub fn solve() -> i64 {
    let mut count = 0;
    for l in 12..=LIMIT {
        let c = find(l);
        if c == 1 {
            count += 1;
        }
    }

    count
}
