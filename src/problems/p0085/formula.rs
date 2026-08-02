//     1   2   3   4   5
// 1:  1   3   6  10  15
// 2:  3   9  18  30  45
// 3:  6  18  36  60  90
// 4: 10  30  60 100 150
// 5: 15  45  90 150 225
fn count_rectangles(width: i64, height: i64) -> i64 {
    let w = width * (width + 1) / 2;
    let h = height * (height + 1) / 2;
    w * h
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

