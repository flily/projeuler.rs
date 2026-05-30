fn d(n: i64) -> i64 {
    if n < 10 {
        return n;
    }

    let mut m = n - 9;
    let mut i = 2_i64;
    let mut num = 10;
    while m > 0 {
        let size = 10_i64.pow(i as u32) - 10_i64.pow((i - 1) as u32);
        if m > size * i {
            m -= size * i;
            num = 10_i64.pow(i as u32);
            i += 1;
            continue;
        }

        let offset = (m - 1) / i;
        let position = (m - 1) % i;
        let number = num + offset;
        return number / 10_i64.pow((i - position - 1) as u32) % 10;
    }

    0
}

pub fn solve() -> i64 {
    let mut result = 1;
    let indexes = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    for x in indexes {
        let y = d(x);
        result *= d(y);
    }
    result
}
