fn count_sum(target: i64, max_num: i64) -> i64 {
    if target == 0 {
        return 1;
    }

    let mut count = 0;
    let mut x = target.min(max_num);
    while x > 0 {
        count += count_sum(target - x, x);
        x -= 1;
    }

    count
}

const TARGET: i64 = 100;

pub fn solve() -> i64 {
    count_sum(TARGET, TARGET) - 1
}
