fn count_rectangles(width: i64, height: i64) -> i64 {
    let mut sum = 0;

    for w in 1..=width {
        for h in 1..=height {
            sum += (width - w + 1) * (height - h + 1);
        }
    }

    sum
}

const TARGET: i64 = 2_000_000;

pub fn solve() -> i64 {
    let mut size = (0, 0);
    let mut min_diff = TARGET;

    for width in 1..100 {
        for height in 1..100 {
            let count = count_rectangles(width, height);
            let diff = (count - TARGET).abs();
            if diff < min_diff {
                min_diff = diff;
                size = (width, height);
            }
        }
    }

    size.0 * size.1
}
