pub fn solve() -> i64 {
    for a in 1..1000 {
        for b in (a + 1)..1000 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }

    // should never reach here
    -1
}
