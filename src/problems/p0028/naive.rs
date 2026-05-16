pub fn solve() -> i64 {
    let mut sum = 1;
    let mut i = 3;
    let mut delta = 2;
    let mut c = 0;

    while i <= 1001 * 1001 {
        sum += i;
        c += 1;
        if c == 4 {
            c = 0;
            delta += 2;
        }

        i += delta;
    }

    sum
}
