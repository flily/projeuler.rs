// This is a template solution file. You can modify it as needed.

pub fn solve() -> i64 {
    let mut reuslt = 1;
    let m = 10_000_000_000;

    for _ in 1..=7830457 {
        reuslt = (reuslt * 2) % m;
    }

    (reuslt * 28433 + 1) % m
}
